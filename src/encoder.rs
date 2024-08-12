use ethers::{abi::Token, types::H160};

#[derive(Debug)]
pub struct Encoder;

impl Encoder {
    pub fn encode_weth_deposit(amount: u128) -> Vec<u8> {
        let abi = ethers::abi::Function {
            name: "deposit".to_string(),
            inputs: vec![],
            constant: None,
            outputs: vec![ethers::abi::Param {
                name: "".to_string(),
                kind: ethers::abi::ParamType::Uint(256),
                internal_type: None,
            }],
            state_mutability: ethers::abi::StateMutability::NonPayable,
        };
        abi.encode_input(&[Token::Uint(amount.into())]).unwrap()
    }

    pub fn encode_weth_withdraw(amount: u128) -> Vec<u8> {
        let abi = ethers::abi::Function {
            name: "withdraw".to_string(),
            inputs: vec![ethers::abi::Param {
                name: "".to_string(),
                kind: ethers::abi::ParamType::Uint(256),
                internal_type: None,
            }],
            outputs: vec![],
            constant: None,
            state_mutability: ethers::abi::StateMutability::NonPayable,
        };
        abi.encode_input(&[Token::Uint(amount.into())]).unwrap()
    }

    pub fn encode_erc20_transfer(to: H160, amount: u128) -> Vec<u8> {
        let abi = ethers::abi::Function {
            name: "transfer".to_string(),
            inputs: vec![ethers::abi::Param {
                name: "".to_string(),
                kind: ethers::abi::ParamType::Address,
                internal_type: None,
            }],
            outputs: vec![],
            constant: None,
            state_mutability: ethers::abi::StateMutability::NonPayable,
        };
        abi.encode_input(&[Token::Address(to), Token::Uint(amount.into())])
            .unwrap()
    }

    // ERC20 Approve
    pub fn encode_erc20_approve(spender: H160, amount: u128) -> Vec<u8> {
        let abi = ethers::abi::Function {
            name: "approve".to_string(),
            inputs: vec![
                ethers::abi::Param {
                    name: "".to_string(),
                    kind: ethers::abi::ParamType::Address,
                    internal_type: None,
                },
                ethers::abi::Param {
                    name: "".to_string(),
                    kind: ethers::abi::ParamType::Uint(256),
                    internal_type: None,
                },
            ],
            outputs: vec![],
            constant: None,
            state_mutability: ethers::abi::StateMutability::NonPayable,
        };
        abi.encode_input(&[Token::Address(spender), Token::Uint(amount.into())])
            .unwrap()
    }
}
