package light

import (
	"context"
	"errors"
	"math/big"
	"sync"
	"time"

	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/common/lru"
	"github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/event"
	"github.com/ethereum/go-ethereum/log"
	"github.com/ethereum/go-ethereum/params"
)

var (
	bodyCacheLimit  = 256
	blockCacheLimit = 256
)

// LightChain represents a stateless light client that can retrieve block headers and bodies on demand.
type LightChain struct {
	odr           OdrBackend
	chainFeed     event.Feed
	chainSideFeed event.Feed
	chainHeadFeed event.Feed
	scope         event.SubscriptionScope

	bodyCache    *lru.Cache[common.Hash, *types.Body]
	bodyRLPCache *lru.Cache[common.Hash, rlp.RawValue]
	blockCache   *lru.Cache[common.Hash, *types.Block]

	quit chan struct{}
	wg   sync.WaitGroup
}

// NewLightChain returns a fully initialized stateless light chain using the given ODR backend.
func NewLightChain(odr OdrBackend) *LightChain {
	return &LightChain{
		odr:           odr,
		chainFeed:     event.Feed{},
		chainSideFeed: event.Feed{},
		chainHeadFeed: event.Feed{},
		scope:         event.NewSubscriptionScope(),
		bodyCache:     lru.NewCache[common.Hash, *types.Body](bodyCacheLimit),
		bodyRLPCache:  lru.NewCache[common.Hash, rlp.RawValue](bodyCacheLimit),
		blockCache:    lru.NewCache[common.Hash, *types.Block](blockCacheLimit),
		quit:          make(chan struct{}),
	}
}

// Odr returns the ODR backend of the chain.
func (lc *LightChain) Odr() OdrBackend {
	return lc.odr
}

// GetBody retrieves a block body (transactions and uncles) from the ODR service by hash, caching it if found.
func (lc *LightChain) GetBody(ctx context.Context, hash common.Hash) (*types.Body, error) {
	// Short circuit if the body's already in the cache, retrieve otherwise
	if cached, ok := lc.bodyCache.Get(hash); ok {
		return cached, nil
	}
	number, err := GetBlockNumber(ctx, lc.odr, hash)
	if err != nil {
		return nil, err
	}
	body, err := GetBody(ctx, lc.odr, hash, number)
	if err != nil {
		return nil, err
	}
	// Cache the found body for next time and return
	lc.bodyCache.Add(hash, body)
	return body, nil
}

// GetBodyRLP retrieves a block body in RLP encoding from the ODR service by hash, caching it if found.
func (lc *LightChain) GetBodyRLP(ctx context.Context, hash common.Hash) (rlp.RawValue, error) {
	// Short circuit if the body's already in the cache, retrieve otherwise
	if cached, ok := lc.bodyRLPCache.Get(hash); ok {
		return cached, nil
	}
	number, err := GetBlockNumber(ctx, lc.odr, hash)
	if err != nil {
		return nil, err
	}
	body, err := GetBodyRLP(ctx, lc.odr, hash, number)
	if err != nil {
		return nil, err
	}
	// Cache the found body for next time and return
	lc.bodyRLPCache.Add(hash, body)
	return body, nil
}

// GetBlock retrieves a block from the ODR service by hash and number, caching it if found.
func (lc *LightChain) GetBlock(ctx context.Context, hash common.Hash, number uint64) (*types.Block, error) {
	// Short circuit if the block's already in the cache, retrieve otherwise
	if block, ok := lc.blockCache.Get(hash); ok {
		return block, nil
	}
	block, err := GetBlock(ctx, lc.odr, hash, number)
	if err != nil {
		return nil, err
	}
	// Cache the found block for next time and return
	lc.blockCache.Add(block.Hash(), block)
	return block, nil
}

// GetBlockByHash retrieves a block from the ODR service by hash, caching it if found.
func (lc *LightChain) GetBlockByHash(ctx context.Context, hash common.Hash) (*types.Block, error) {
	number, err := GetBlockNumber(ctx, lc.odr, hash)
	if err != nil {
		return nil, err
	}
	return lc.GetBlock(ctx, hash, number)
}

// GetBlockByNumber retrieves a block from the ODR service by number, caching it if found.
func (lc *LightChain) GetBlockByNumber(ctx context.Context, number uint64) (*types.Block, error) {
	hash, err := GetCanonicalHash(ctx, lc.odr, number)
	if hash == (common.Hash{}) || err != nil {
		return nil, err
	}
	return lc.GetBlock(ctx, hash, number)
}

// Stop stops the blockchain service.
func (lc *LightChain) Stop() {
	close(lc.quit)
	lc.wg.Wait()
	log.Info("Blockchain stopped")
}

// SubscribeChainEvent registers a subscription of ChainEvent.
func (lc *LightChain) SubscribeChainEvent(ch chan<- core.ChainEvent) event.Subscription {
	return lc.scope.Track(lc.chainFeed.Subscribe(ch))
}

// SubscribeChainHeadEvent registers a subscription of ChainHeadEvent.
func (lc *LightChain) SubscribeChainHeadEvent(ch chan<- core.ChainHeadEvent) event.Subscription {
	return lc.scope.Track(lc.chainHeadFeed.Subscribe(ch))
}

// SubscribeChainSideEvent registers a subscription of ChainSideEvent.
func (lc *LightChain) SubscribeChainSideEvent(ch chan<- core.ChainSideEvent) event.Subscription {
	return lc.scope.Track(lc.chainSideFeed.Subscribe(ch))
}

// SubscribeLogsEvent implements the interface of filters.Backend.
// LightChain does not send logs events, so return an empty subscription.
func (lc *LightChain) SubscribeLogsEvent(ch chan<- []*types.Log) event.Subscription {
	return lc.scope.Track(new(event.Feed).Subscribe(ch))
}

// SubscribeRemovedLogsEvent implements the interface of filters.Backend.
// LightChain does not send core.RemovedLogsEvent, so return an empty subscription.
func (lc *LightChain) SubscribeRemovedLogsEvent(ch chan<- core.RemovedLogsEvent) event.Subscription {
	return lc.scope.Track(new(event.Feed).Subscribe(ch))
}

// HeaderWithAggregatedSubvector is an extension of types.Header that includes an aggregated subvector commitment.
type HeaderWithAggregatedSubvector struct {
	types.Header
	AggregatedSubvector common.Hash `json:"aggregatedSubvector" rlp:"size=32"`
}

// NewHeaderWithAggregatedSubvector creates a new header with the given fields.
func NewHeaderWithAggregatedSubvector(parentHash, unclesHash, coinbase common.Address, root common.Hash, aggregatedSubvector common.Hash, txHash, receiptHash common.Hash, bloom types.Bloom, difficulty *big.Int, number, gasLimit, gasUsed uint64, time, extra []byte, mixDigest common.Hash, nonce types.BlockNonce) *HeaderWithAggregatedSubvector {
	return &HeaderWithAggregatedSubvector{
		Header: types.Header{
			ParentHash:  parentHash,
			UncleHash:   unclesHash,
			Coinbase:    coinbase,
			Root:        root,
			TxHash:      txHash,
			ReceiptHash: receiptHash,
			Bloom:       bloom,
			Difficulty:  difficulty,
			Number:      number,
			GasLimit:    gasLimit,
			GasUsed:     gasUsed,
			Time:        time,
			Extra:       extra,
			MixDigest:   mixDigest,
			Nonce:       nonce,
		},
		AggregatedSubvector: aggregatedSubvector,
	}
}
