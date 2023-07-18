GOENV=GO111MODULE=on
GO=${GOENV} go

COVERAGE_OUT=/tmp/coverage.out
PACKAGE=./...

TEST_CLAUSE= $(if ${TEST}, -run ${TEST})

DOCTOOLS=docker run --rm  -v "$$(pwd)":"$$(pwd)" -w "$$(pwd)" doctools:latest


build:
	${GO} build ./...

test:
	${GO} test -short ${TEST_CLAUSE} ./...

clean:
	${GO} clean -cache -modcache -i -r
