pragma solidity ^0.4.10;

contract Bank {
    mapping(address => uint) balances;
    uint public totalSupply;

    modifier hasBalance {
        require(balance[msg.sender] > 0);
        _;
    }

    function deposit() public payable {
        balances[msg.sender] += msg.value;
        totalSupply += msg.value;
        assert(this.balances >= totalSupply);
        assert(balances[msg.sender] >= msg.sender);
    }

    function withdrawAll() public hasBalance {
        //check balancs is more than 0
        require(balances[msg.sender] >= 0);
        uint amountToWithdraw = balances[msg.sender];
        balances[msg.sender] = 0;
        msg.sender.transfer(amountToWithdraw);
        //check balance is 0
        assert(balance[msg.sender] == 0);
    }



}
