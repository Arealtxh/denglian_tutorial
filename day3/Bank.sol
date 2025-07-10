//SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

contract Bank {
    mapping(address => uint256) public BankAmount;
    address public owner;
    address[3] public TopDepositors;
    uint256[3] public TopDepositAmounts;

    constructor() {
        owner = msg.sender;
    }

    function RecordBalance() external payable {
        require(msg.value > 0, "Deposit amount must be greater than 0");
        BankAmount[msg.sender] += msg.value;
        updateTopDepositors(msg.sender, BankAmount[msg.sender]);
    }

    function withdraw(uint256 amount) external {
        require(msg.sender == owner, "Only owner can withdraw");
        require(
            address(this).balance >= amount,
            "Insufficient contract balance"
        );
        payable(owner).transfer(amount);
    }

    function updateTopDepositors(
        address depositor,
        uint256 depositoramount
    ) private {
        for (uint256 i = 0; i < 3; i++) {
            if (depositor == TopDepositors[i]) {
                TopDepositAmounts[i] = depositoramount;
                sortTopDepositors();
                return;
            }
        }
        if (depositoramount > TopDepositAmounts[2]) {
            TopDepositAmounts[2] = depositoramount;
            TopDepositors[2] = depositor;
            sortTopDepositors();
        } else return;
    }

    //sort top3 depositor
    function sortTopDepositors() private {
        for (uint256 i = 0; i < 2; i++) {
            for (uint256 j = 0; j < 2 - i; j++) {
                if (TopDepositAmounts[j] < TopDepositAmounts[j + 1]) {
                    (TopDepositAmounts[j], TopDepositAmounts[j + 1]) = (
                        TopDepositAmounts[j + 1],
                        TopDepositAmounts[j]
                    );
                    (TopDepositors[j], TopDepositors[j + 1]) = (
                        TopDepositors[j + 1],
                        TopDepositors[j]
                    );
                }
            }
        }
    }

    function getTopDepositors()
        external
        view
        returns (address[3] memory, uint256[3] memory)
    {
        return (TopDepositors, TopDepositAmounts);
    }
}
