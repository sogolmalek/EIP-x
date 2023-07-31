contract MyToken {
    function transfer(address _to, uint256 _amount) external returns (bool);
    function balanceOf(address _owner) external view returns (uint256);
}
