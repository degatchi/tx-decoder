pub use b_contract::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod b_contract {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
    #![allow(unused_imports)]
    use ethers::contract::{
        builders::{ContractCall, Event},
        Contract, Lazy,
    };
    use ethers::core::{
        abi::{Abi, Detokenize, InvalidOutputType, Token, Tokenizable},
        types::*,
    };
    use ethers::providers::Middleware;
    #[doc = "b_contract was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static B_CONTRACT_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"_vault\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"address\",\n                \"name\": \"_router\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"address\",\n                \"name\": \"_weth\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"_depositFee\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"_minExecutionFee\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"constructor\"\n    },\n    {\n        \"anonymous\": false,\n        \"inputs\": [\n            {\n                \"indexed\": true,\n                \"internalType\": \"address\",\n                \"name\": \"account\",\n                \"type\": \"address\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"address[]\",\n                \"name\": \"path\",\n                \"type\": \"address[]\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"address\",\n                \"name\": \"indexToken\",\n                \"type\": \"address\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"collateralDelta\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"sizeDelta\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"bool\",\n                \"name\": \"isLong\",\n                \"type\": \"bool\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"address\",\n                \"name\": \"receiver\",\n                \"type\": \"address\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"acceptablePrice\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"minOut\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"executionFee\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"blockGap\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"timeGap\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"CancelDecreasePosition\",\n        \"type\": \"event\"\n    },\n    {\n        \"anonymous\": false,\n        \"inputs\": [\n            {\n                \"indexed\": true,\n                \"internalType\": \"address\",\n                \"name\": \"account\",\n                \"type\": \"address\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"address[]\",\n                \"name\": \"path\",\n                \"type\": \"address[]\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"address\",\n                \"name\": \"indexToken\",\n                \"type\": \"address\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"amountIn\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"minOut\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"sizeDelta\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"bool\",\n                \"name\": \"isLong\",\n                \"type\": \"bool\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"acceptablePrice\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"executionFee\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"blockGap\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"timeGap\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"CancelIncreasePosition\",\n        \"type\": \"event\"\n    },\n    {\n        \"anonymous\": false,\n        \"inputs\": [\n            {\n                \"indexed\": true,\n                \"internalType\": \"address\",\n                \"name\": \"account\",\n                \"type\": \"address\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"address[]\",\n                \"name\": \"path\",\n                \"type\": \"address[]\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"address\",\n                \"name\": \"indexToken\",\n                \"type\": \"address\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"collateralDelta\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"sizeDelta\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"bool\",\n                \"name\": \"isLong\",\n                \"type\": \"bool\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"address\",\n                \"name\": \"receiver\",\n                \"type\": \"address\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"acceptablePrice\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"minOut\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"executionFee\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"index\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"blockNumber\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"blockTime\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"CreateDecreasePosition\",\n        \"type\": \"event\"\n    },\n    {\n        \"anonymous\": false,\n        \"inputs\": [\n            {\n                \"indexed\": true,\n                \"internalType\": \"address\",\n                \"name\": \"account\",\n                \"type\": \"address\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"address[]\",\n                \"name\": \"path\",\n                \"type\": \"address[]\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"address\",\n                \"name\": \"indexToken\",\n                \"type\": \"address\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"amountIn\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"minOut\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"sizeDelta\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"bool\",\n                \"name\": \"isLong\",\n                \"type\": \"bool\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"acceptablePrice\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"executionFee\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"index\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"blockNumber\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"blockTime\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"gasPrice\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"CreateIncreasePosition\",\n        \"type\": \"event\"\n    },\n    {\n        \"anonymous\": false,\n        \"inputs\": [\n            {\n                \"indexed\": false,\n                \"internalType\": \"address\",\n                \"name\": \"account\",\n                \"type\": \"address\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"sizeDelta\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"marginFeeBasisPoints\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"bytes32\",\n                \"name\": \"referralCode\",\n                \"type\": \"bytes32\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"address\",\n                \"name\": \"referrer\",\n                \"type\": \"address\"\n            }\n        ],\n        \"name\": \"DecreasePositionReferral\",\n        \"type\": \"event\"\n    },\n    {\n        \"anonymous\": false,\n        \"inputs\": [\n            {\n                \"indexed\": true,\n                \"internalType\": \"address\",\n                \"name\": \"account\",\n                \"type\": \"address\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"address[]\",\n                \"name\": \"path\",\n                \"type\": \"address[]\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"address\",\n                \"name\": \"indexToken\",\n                \"type\": \"address\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"collateralDelta\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"sizeDelta\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"bool\",\n                \"name\": \"isLong\",\n                \"type\": \"bool\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"address\",\n                \"name\": \"receiver\",\n                \"type\": \"address\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"acceptablePrice\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"minOut\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"executionFee\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"blockGap\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"timeGap\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"ExecuteDecreasePosition\",\n        \"type\": \"event\"\n    },\n    {\n        \"anonymous\": false,\n        \"inputs\": [\n            {\n                \"indexed\": true,\n                \"internalType\": \"address\",\n                \"name\": \"account\",\n                \"type\": \"address\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"address[]\",\n                \"name\": \"path\",\n                \"type\": \"address[]\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"address\",\n                \"name\": \"indexToken\",\n                \"type\": \"address\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"amountIn\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"minOut\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"sizeDelta\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"bool\",\n                \"name\": \"isLong\",\n                \"type\": \"bool\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"acceptablePrice\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"executionFee\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"blockGap\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"timeGap\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"ExecuteIncreasePosition\",\n        \"type\": \"event\"\n    },\n    {\n        \"anonymous\": false,\n        \"inputs\": [\n            {\n                \"indexed\": false,\n                \"internalType\": \"address\",\n                \"name\": \"account\",\n                \"type\": \"address\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"sizeDelta\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"marginFeeBasisPoints\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"bytes32\",\n                \"name\": \"referralCode\",\n                \"type\": \"bytes32\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"address\",\n                \"name\": \"referrer\",\n                \"type\": \"address\"\n            }\n        ],\n        \"name\": \"IncreasePositionReferral\",\n        \"type\": \"event\"\n    },\n    {\n        \"anonymous\": false,\n        \"inputs\": [\n            {\n                \"indexed\": false,\n                \"internalType\": \"address\",\n                \"name\": \"admin\",\n                \"type\": \"address\"\n            }\n        ],\n        \"name\": \"SetAdmin\",\n        \"type\": \"event\"\n    },\n    {\n        \"anonymous\": false,\n        \"inputs\": [\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"minBlockDelayKeeper\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"minTimeDelayPublic\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"maxTimeDelay\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"SetDelayValues\",\n        \"type\": \"event\"\n    },\n    {\n        \"anonymous\": false,\n        \"inputs\": [\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"depositFee\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"SetDepositFee\",\n        \"type\": \"event\"\n    },\n    {\n        \"anonymous\": false,\n        \"inputs\": [\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"increasePositionBufferBps\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"SetIncreasePositionBufferBps\",\n        \"type\": \"event\"\n    },\n    {\n        \"anonymous\": false,\n        \"inputs\": [\n            {\n                \"indexed\": false,\n                \"internalType\": \"bool\",\n                \"name\": \"isLeverageEnabled\",\n                \"type\": \"bool\"\n            }\n        ],\n        \"name\": \"SetIsLeverageEnabled\",\n        \"type\": \"event\"\n    },\n    {\n        \"anonymous\": false,\n        \"inputs\": [\n            {\n                \"indexed\": false,\n                \"internalType\": \"address[]\",\n                \"name\": \"tokens\",\n                \"type\": \"address[]\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256[]\",\n                \"name\": \"longSizes\",\n                \"type\": \"uint256[]\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256[]\",\n                \"name\": \"shortSizes\",\n                \"type\": \"uint256[]\"\n            }\n        ],\n        \"name\": \"SetMaxGlobalSizes\",\n        \"type\": \"event\"\n    },\n    {\n        \"anonymous\": false,\n        \"inputs\": [\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"minExecutionFee\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"SetMinExecutionFee\",\n        \"type\": \"event\"\n    },\n    {\n        \"anonymous\": false,\n        \"inputs\": [\n            {\n                \"indexed\": true,\n                \"internalType\": \"address\",\n                \"name\": \"account\",\n                \"type\": \"address\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"bool\",\n                \"name\": \"isActive\",\n                \"type\": \"bool\"\n            }\n        ],\n        \"name\": \"SetPositionKeeper\",\n        \"type\": \"event\"\n    },\n    {\n        \"anonymous\": false,\n        \"inputs\": [\n            {\n                \"indexed\": false,\n                \"internalType\": \"address\",\n                \"name\": \"referralStorage\",\n                \"type\": \"address\"\n            }\n        ],\n        \"name\": \"SetReferralStorage\",\n        \"type\": \"event\"\n    },\n    {\n        \"anonymous\": false,\n        \"inputs\": [\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"increasePositionRequestKeysStart\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"decreasePositionRequestKeysStart\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"SetRequestKeysStartValues\",\n        \"type\": \"event\"\n    },\n    {\n        \"anonymous\": false,\n        \"inputs\": [\n            {\n                \"indexed\": false,\n                \"internalType\": \"address\",\n                \"name\": \"token\",\n                \"type\": \"address\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"address\",\n                \"name\": \"receiver\",\n                \"type\": \"address\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"amount\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"WithdrawFees\",\n        \"type\": \"event\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"BASIS_POINTS_DIVISOR\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"admin\",\n        \"outputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"\",\n                \"type\": \"address\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"_token\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"address\",\n                \"name\": \"_spender\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"_amount\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"approve\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"bytes32\",\n                \"name\": \"_key\",\n                \"type\": \"bytes32\"\n            },\n            {\n                \"internalType\": \"address payable\",\n                \"name\": \"_executionFeeReceiver\",\n                \"type\": \"address\"\n            }\n        ],\n        \"name\": \"cancelDecreasePosition\",\n        \"outputs\": [\n            {\n                \"internalType\": \"bool\",\n                \"name\": \"\",\n                \"type\": \"bool\"\n            }\n        ],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"bytes32\",\n                \"name\": \"_key\",\n                \"type\": \"bytes32\"\n            },\n            {\n                \"internalType\": \"address payable\",\n                \"name\": \"_executionFeeReceiver\",\n                \"type\": \"address\"\n            }\n        ],\n        \"name\": \"cancelIncreasePosition\",\n        \"outputs\": [\n            {\n                \"internalType\": \"bool\",\n                \"name\": \"\",\n                \"type\": \"bool\"\n            }\n        ],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address[]\",\n                \"name\": \"_path\",\n                \"type\": \"address[]\"\n            },\n            {\n                \"internalType\": \"address\",\n                \"name\": \"_indexToken\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"_collateralDelta\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"_sizeDelta\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"bool\",\n                \"name\": \"_isLong\",\n                \"type\": \"bool\"\n            },\n            {\n                \"internalType\": \"address\",\n                \"name\": \"_receiver\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"_acceptablePrice\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"_minOut\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"_executionFee\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"bool\",\n                \"name\": \"_withdrawETH\",\n                \"type\": \"bool\"\n            }\n        ],\n        \"name\": \"createDecreasePosition\",\n        \"outputs\": [],\n        \"stateMutability\": \"payable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address[]\",\n                \"name\": \"_path\",\n                \"type\": \"address[]\"\n            },\n            {\n                \"internalType\": \"address\",\n                \"name\": \"_indexToken\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"_amountIn\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"_minOut\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"_sizeDelta\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"bool\",\n                \"name\": \"_isLong\",\n                \"type\": \"bool\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"_acceptablePrice\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"_executionFee\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"bytes32\",\n                \"name\": \"_referralCode\",\n                \"type\": \"bytes32\"\n            }\n        ],\n        \"name\": \"createIncreasePosition\",\n        \"outputs\": [],\n        \"stateMutability\": \"payable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address[]\",\n                \"name\": \"_path\",\n                \"type\": \"address[]\"\n            },\n            {\n                \"internalType\": \"address\",\n                \"name\": \"_indexToken\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"_minOut\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"_sizeDelta\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"bool\",\n                \"name\": \"_isLong\",\n                \"type\": \"bool\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"_acceptablePrice\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"_executionFee\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"bytes32\",\n                \"name\": \"_referralCode\",\n                \"type\": \"bytes32\"\n            }\n        ],\n        \"name\": \"createIncreasePositionETH\",\n        \"outputs\": [],\n        \"stateMutability\": \"payable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"decreasePositionRequestKeys\",\n        \"outputs\": [\n            {\n                \"internalType\": \"bytes32\",\n                \"name\": \"\",\n                \"type\": \"bytes32\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"decreasePositionRequestKeysStart\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"bytes32\",\n                \"name\": \"\",\n                \"type\": \"bytes32\"\n            }\n        ],\n        \"name\": \"decreasePositionRequests\",\n        \"outputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"account\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"address\",\n                \"name\": \"indexToken\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"collateralDelta\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"sizeDelta\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"bool\",\n                \"name\": \"isLong\",\n                \"type\": \"bool\"\n            },\n            {\n                \"internalType\": \"address\",\n                \"name\": \"receiver\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"acceptablePrice\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"minOut\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"executionFee\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"blockNumber\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"blockTime\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"bool\",\n                \"name\": \"withdrawETH\",\n                \"type\": \"bool\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"\",\n                \"type\": \"address\"\n            }\n        ],\n        \"name\": \"decreasePositionsIndex\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"depositFee\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"bytes32\",\n                \"name\": \"_key\",\n                \"type\": \"bytes32\"\n            },\n            {\n                \"internalType\": \"address payable\",\n                \"name\": \"_executionFeeReceiver\",\n                \"type\": \"address\"\n            }\n        ],\n        \"name\": \"executeDecreasePosition\",\n        \"outputs\": [\n            {\n                \"internalType\": \"bool\",\n                \"name\": \"\",\n                \"type\": \"bool\"\n            }\n        ],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"_endIndex\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"address payable\",\n                \"name\": \"_executionFeeReceiver\",\n                \"type\": \"address\"\n            }\n        ],\n        \"name\": \"executeDecreasePositions\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"bytes32\",\n                \"name\": \"_key\",\n                \"type\": \"bytes32\"\n            },\n            {\n                \"internalType\": \"address payable\",\n                \"name\": \"_executionFeeReceiver\",\n                \"type\": \"address\"\n            }\n        ],\n        \"name\": \"executeIncreasePosition\",\n        \"outputs\": [\n            {\n                \"internalType\": \"bool\",\n                \"name\": \"\",\n                \"type\": \"bool\"\n            }\n        ],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"_endIndex\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"address payable\",\n                \"name\": \"_executionFeeReceiver\",\n                \"type\": \"address\"\n            }\n        ],\n        \"name\": \"executeIncreasePositions\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"\",\n                \"type\": \"address\"\n            }\n        ],\n        \"name\": \"feeReserves\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"bytes32\",\n                \"name\": \"_key\",\n                \"type\": \"bytes32\"\n            }\n        ],\n        \"name\": \"getDecreasePositionRequestPath\",\n        \"outputs\": [\n            {\n                \"internalType\": \"address[]\",\n                \"name\": \"\",\n                \"type\": \"address[]\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"bytes32\",\n                \"name\": \"_key\",\n                \"type\": \"bytes32\"\n            }\n        ],\n        \"name\": \"getIncreasePositionRequestPath\",\n        \"outputs\": [\n            {\n                \"internalType\": \"address[]\",\n                \"name\": \"\",\n                \"type\": \"address[]\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"_account\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"_index\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"getRequestKey\",\n        \"outputs\": [\n            {\n                \"internalType\": \"bytes32\",\n                \"name\": \"\",\n                \"type\": \"bytes32\"\n            }\n        ],\n        \"stateMutability\": \"pure\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"getRequestQueueLengths\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"gov\",\n        \"outputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"\",\n                \"type\": \"address\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"increasePositionBufferBps\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"increasePositionRequestKeys\",\n        \"outputs\": [\n            {\n                \"internalType\": \"bytes32\",\n                \"name\": \"\",\n                \"type\": \"bytes32\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"increasePositionRequestKeysStart\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"bytes32\",\n                \"name\": \"\",\n                \"type\": \"bytes32\"\n            }\n        ],\n        \"name\": \"increasePositionRequests\",\n        \"outputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"account\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"address\",\n                \"name\": \"indexToken\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"amountIn\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"minOut\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"sizeDelta\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"bool\",\n                \"name\": \"isLong\",\n                \"type\": \"bool\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"acceptablePrice\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"executionFee\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"blockNumber\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"blockTime\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"bool\",\n                \"name\": \"hasCollateralInETH\",\n                \"type\": \"bool\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"\",\n                \"type\": \"address\"\n            }\n        ],\n        \"name\": \"increasePositionsIndex\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"isLeverageEnabled\",\n        \"outputs\": [\n            {\n                \"internalType\": \"bool\",\n                \"name\": \"\",\n                \"type\": \"bool\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"\",\n                \"type\": \"address\"\n            }\n        ],\n        \"name\": \"isPositionKeeper\",\n        \"outputs\": [\n            {\n                \"internalType\": \"bool\",\n                \"name\": \"\",\n                \"type\": \"bool\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"\",\n                \"type\": \"address\"\n            }\n        ],\n        \"name\": \"maxGlobalLongSizes\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"\",\n                \"type\": \"address\"\n            }\n        ],\n        \"name\": \"maxGlobalShortSizes\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"maxTimeDelay\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"minBlockDelayKeeper\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"minExecutionFee\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"minTimeDelayPublic\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"referralStorage\",\n        \"outputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"\",\n                \"type\": \"address\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"router\",\n        \"outputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"\",\n                \"type\": \"address\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address payable\",\n                \"name\": \"_receiver\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"_amount\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"sendValue\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"_admin\",\n                \"type\": \"address\"\n            }\n        ],\n        \"name\": \"setAdmin\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"_minBlockDelayKeeper\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"_minTimeDelayPublic\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"_maxTimeDelay\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"setDelayValues\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"_depositFee\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"setDepositFee\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"_gov\",\n                \"type\": \"address\"\n            }\n        ],\n        \"name\": \"setGov\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"_increasePositionBufferBps\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"setIncreasePositionBufferBps\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"bool\",\n                \"name\": \"_isLeverageEnabled\",\n                \"type\": \"bool\"\n            }\n        ],\n        \"name\": \"setIsLeverageEnabled\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address[]\",\n                \"name\": \"_tokens\",\n                \"type\": \"address[]\"\n            },\n            {\n                \"internalType\": \"uint256[]\",\n                \"name\": \"_longSizes\",\n                \"type\": \"uint256[]\"\n            },\n            {\n                \"internalType\": \"uint256[]\",\n                \"name\": \"_shortSizes\",\n                \"type\": \"uint256[]\"\n            }\n        ],\n        \"name\": \"setMaxGlobalSizes\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"_minExecutionFee\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"setMinExecutionFee\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"_account\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"bool\",\n                \"name\": \"_isActive\",\n                \"type\": \"bool\"\n            }\n        ],\n        \"name\": \"setPositionKeeper\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"_referralStorage\",\n                \"type\": \"address\"\n            }\n        ],\n        \"name\": \"setReferralStorage\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"_increasePositionRequestKeysStart\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"_decreasePositionRequestKeysStart\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"setRequestKeysStartValues\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"vault\",\n        \"outputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"\",\n                \"type\": \"address\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"weth\",\n        \"outputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"\",\n                \"type\": \"address\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"_token\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"address\",\n                \"name\": \"_receiver\",\n                \"type\": \"address\"\n            }\n        ],\n        \"name\": \"withdrawFees\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"stateMutability\": \"payable\",\n        \"type\": \"receive\"\n    }\n]") . expect ("invalid abi")
        });
    pub struct b_contract<M>(ethers::contract::Contract<M>);
    impl<M> Clone for b_contract<M> {
        fn clone(&self) -> Self {
            b_contract(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for b_contract<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for b_contract<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(b_contract))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> b_contract<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), B_CONTRACT_ABI.clone(), client).into()
        }
        #[doc = "Calls the contract's `BASIS_POINTS_DIVISOR` (0x126082cf) function"]
        pub fn basis_points_divisor(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([18, 96, 130, 207], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `admin` (0xf851a440) function"]
        pub fn admin(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([248, 81, 164, 64], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `approve` (0xe1f21c67) function"]
        pub fn approve(
            &self,
            token: ethers::core::types::Address,
            spender: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([225, 242, 28, 103], (token, spender, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `cancelDecreasePosition` (0x60a362e2) function"]
        pub fn cancel_decrease_position(
            &self,
            key: [u8; 32],
            execution_fee_receiver: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([96, 163, 98, 226], (key, execution_fee_receiver))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `cancelIncreasePosition` (0x225fc9fd) function"]
        pub fn cancel_increase_position(
            &self,
            key: [u8; 32],
            execution_fee_receiver: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([34, 95, 201, 253], (key, execution_fee_receiver))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `createDecreasePosition` (0xe70dd2fc) function"]
        pub fn create_decrease_position(
            &self,
            path: ::std::vec::Vec<ethers::core::types::Address>,
            index_token: ethers::core::types::Address,
            collateral_delta: ethers::core::types::U256,
            size_delta: ethers::core::types::U256,
            is_long: bool,
            receiver: ethers::core::types::Address,
            acceptable_price: ethers::core::types::U256,
            min_out: ethers::core::types::U256,
            execution_fee: ethers::core::types::U256,
            withdraw_eth: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [231, 13, 210, 252],
                    (
                        path,
                        index_token,
                        collateral_delta,
                        size_delta,
                        is_long,
                        receiver,
                        acceptable_price,
                        min_out,
                        execution_fee,
                        withdraw_eth,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `createIncreasePosition` (0xc9874170) function"]
        pub fn create_increase_position(
            &self,
            path: ::std::vec::Vec<ethers::core::types::Address>,
            index_token: ethers::core::types::Address,
            amount_in: ethers::core::types::U256,
            min_out: ethers::core::types::U256,
            size_delta: ethers::core::types::U256,
            is_long: bool,
            acceptable_price: ethers::core::types::U256,
            execution_fee: ethers::core::types::U256,
            referral_code: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [201, 135, 65, 112],
                    (
                        path,
                        index_token,
                        amount_in,
                        min_out,
                        size_delta,
                        is_long,
                        acceptable_price,
                        execution_fee,
                        referral_code,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `createIncreasePositionETH` (0x332e0382) function"]
        pub fn create_increase_position_eth(
            &self,
            path: ::std::vec::Vec<ethers::core::types::Address>,
            index_token: ethers::core::types::Address,
            min_out: ethers::core::types::U256,
            size_delta: ethers::core::types::U256,
            is_long: bool,
            acceptable_price: ethers::core::types::U256,
            execution_fee: ethers::core::types::U256,
            referral_code: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [51, 46, 3, 130],
                    (
                        path,
                        index_token,
                        min_out,
                        size_delta,
                        is_long,
                        acceptable_price,
                        execution_fee,
                        referral_code,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `decreasePositionRequestKeys` (0x4278555f) function"]
        pub fn decrease_position_request_keys(
            &self,
            p0: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([66, 120, 85, 95], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `decreasePositionRequestKeysStart` (0x1bca8cf0) function"]
        pub fn decrease_position_request_keys_start(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([27, 202, 140, 240], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `decreasePositionRequests` (0x1f285106) function"]
        pub fn decrease_position_requests(
            &self,
            p0: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                ethers::core::types::Address,
                ethers::core::types::Address,
                ethers::core::types::U256,
                ethers::core::types::U256,
                bool,
                ethers::core::types::Address,
                ethers::core::types::U256,
                ethers::core::types::U256,
                ethers::core::types::U256,
                ethers::core::types::U256,
                ethers::core::types::U256,
                bool,
            ),
        > {
            self.0
                .method_hash([31, 40, 81, 6], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `decreasePositionsIndex` (0xfa444577) function"]
        pub fn decrease_positions_index(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([250, 68, 69, 119], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `depositFee` (0x67a52793) function"]
        pub fn deposit_fee(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([103, 165, 39, 147], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `executeDecreasePosition` (0x0d4d003d) function"]
        pub fn execute_decrease_position(
            &self,
            key: [u8; 32],
            execution_fee_receiver: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([13, 77, 0, 61], (key, execution_fee_receiver))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `executeDecreasePositions` (0xf3883d8b) function"]
        pub fn execute_decrease_positions(
            &self,
            end_index: ethers::core::types::U256,
            execution_fee_receiver: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([243, 136, 61, 139], (end_index, execution_fee_receiver))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `executeIncreasePosition` (0x27b42c0f) function"]
        pub fn execute_increase_position(
            &self,
            key: [u8; 32],
            execution_fee_receiver: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([39, 180, 44, 15], (key, execution_fee_receiver))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `executeIncreasePositions` (0x9a208100) function"]
        pub fn execute_increase_positions(
            &self,
            end_index: ethers::core::types::U256,
            execution_fee_receiver: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([154, 32, 129, 0], (end_index, execution_fee_receiver))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `feeReserves` (0x1ce9cb8f) function"]
        pub fn fee_reserves(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([28, 233, 203, 143], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getDecreasePositionRequestPath` (0x5d5c22e8) function"]
        pub fn get_decrease_position_request_path(
            &self,
            key: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<ethers::core::types::Address>,
        > {
            self.0
                .method_hash([93, 92, 34, 232], key)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getIncreasePositionRequestPath` (0x95e9bbd7) function"]
        pub fn get_increase_position_request_path(
            &self,
            key: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<ethers::core::types::Address>,
        > {
            self.0
                .method_hash([149, 233, 187, 215], key)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getRequestKey` (0x62f8a3fe) function"]
        pub fn get_request_key(
            &self,
            account: ethers::core::types::Address,
            index: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([98, 248, 163, 254], (account, index))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getRequestQueueLengths` (0xf2cea6a5) function"]
        pub fn get_request_queue_lengths(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                ethers::core::types::U256,
                ethers::core::types::U256,
                ethers::core::types::U256,
                ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([242, 206, 166, 165], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `gov` (0x12d43a51) function"]
        pub fn gov(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([18, 212, 58, 81], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `increasePositionBufferBps` (0x98d1e03a) function"]
        pub fn increase_position_buffer_bps(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([152, 209, 224, 58], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `increasePositionRequestKeys` (0x04225954) function"]
        pub fn increase_position_request_keys(
            &self,
            p0: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([4, 34, 89, 84], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `increasePositionRequestKeysStart` (0x9b578620) function"]
        pub fn increase_position_request_keys_start(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([155, 87, 134, 32], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `increasePositionRequests` (0xfaf990f3) function"]
        pub fn increase_position_requests(
            &self,
            p0: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                ethers::core::types::Address,
                ethers::core::types::Address,
                ethers::core::types::U256,
                ethers::core::types::U256,
                ethers::core::types::U256,
                bool,
                ethers::core::types::U256,
                ethers::core::types::U256,
                ethers::core::types::U256,
                ethers::core::types::U256,
                bool,
            ),
        > {
            self.0
                .method_hash([250, 249, 144, 243], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `increasePositionsIndex` (0x633451de) function"]
        pub fn increase_positions_index(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([99, 52, 81, 222], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isLeverageEnabled` (0x3e72a262) function"]
        pub fn is_leverage_enabled(&self) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([62, 114, 162, 98], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isPositionKeeper` (0x36eba48a) function"]
        pub fn is_position_keeper(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([54, 235, 164, 138], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `maxGlobalLongSizes` (0x1045c74e) function"]
        pub fn max_global_long_sizes(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([16, 69, 199, 78], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `maxGlobalShortSizes` (0x9698d25a) function"]
        pub fn max_global_short_sizes(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([150, 152, 210, 90], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `maxTimeDelay` (0xcb0269c9) function"]
        pub fn max_time_delay(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([203, 2, 105, 201], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `minBlockDelayKeeper` (0x5841fcaa) function"]
        pub fn min_block_delay_keeper(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([88, 65, 252, 170], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `minExecutionFee` (0x63ae2103) function"]
        pub fn min_execution_fee(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([99, 174, 33, 3], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `minTimeDelayPublic` (0x3a2a80c7) function"]
        pub fn min_time_delay_public(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([58, 42, 128, 199], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `referralStorage` (0x006cc35e) function"]
        pub fn referral_storage(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([0, 108, 195, 94], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `router` (0xf887ea40) function"]
        pub fn router(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([248, 135, 234, 64], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `sendValue` (0x24a084df) function"]
        pub fn send_value(
            &self,
            receiver: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([36, 160, 132, 223], (receiver, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setAdmin` (0x704b6c02) function"]
        pub fn set_admin(
            &self,
            admin: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([112, 75, 108, 2], admin)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setDelayValues` (0x4067b132) function"]
        pub fn set_delay_values(
            &self,
            min_block_delay_keeper: ethers::core::types::U256,
            min_time_delay_public: ethers::core::types::U256,
            max_time_delay: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [64, 103, 177, 50],
                    (
                        min_block_delay_keeper,
                        min_time_delay_public,
                        max_time_delay,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setDepositFee` (0x490ae210) function"]
        pub fn set_deposit_fee(
            &self,
            deposit_fee: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([73, 10, 226, 16], deposit_fee)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setGov` (0xcfad57a2) function"]
        pub fn set_gov(
            &self,
            gov: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([207, 173, 87, 162], gov)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setIncreasePositionBufferBps` (0x233bfe3b) function"]
        pub fn set_increase_position_buffer_bps(
            &self,
            increase_position_buffer_bps: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([35, 59, 254, 59], increase_position_buffer_bps)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setIsLeverageEnabled` (0x7c2eb9f7) function"]
        pub fn set_is_leverage_enabled(
            &self,
            is_leverage_enabled: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([124, 46, 185, 247], is_leverage_enabled)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setMaxGlobalSizes` (0xef12c67e) function"]
        pub fn set_max_global_sizes(
            &self,
            tokens: ::std::vec::Vec<ethers::core::types::Address>,
            long_sizes: ::std::vec::Vec<ethers::core::types::U256>,
            short_sizes: ::std::vec::Vec<ethers::core::types::U256>,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([239, 18, 198, 126], (tokens, long_sizes, short_sizes))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setMinExecutionFee` (0xfc2cee62) function"]
        pub fn set_min_execution_fee(
            &self,
            min_execution_fee: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([252, 44, 238, 98], min_execution_fee)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setPositionKeeper` (0x3422ead1) function"]
        pub fn set_position_keeper(
            &self,
            account: ethers::core::types::Address,
            is_active: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([52, 34, 234, 209], (account, is_active))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setReferralStorage` (0xae4d7f9a) function"]
        pub fn set_referral_storage(
            &self,
            referral_storage: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([174, 77, 127, 154], referral_storage)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setRequestKeysStartValues` (0x308aa81f) function"]
        pub fn set_request_keys_start_values(
            &self,
            increase_position_request_keys_start: ethers::core::types::U256,
            decrease_position_request_keys_start: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [48, 138, 168, 31],
                    (
                        increase_position_request_keys_start,
                        decrease_position_request_keys_start,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `vault` (0xfbfa77cf) function"]
        pub fn vault(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([251, 250, 119, 207], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `weth` (0x3fc8cef3) function"]
        pub fn weth(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([63, 200, 206, 243], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `withdrawFees` (0xf2555278) function"]
        pub fn withdraw_fees(
            &self,
            token: ethers::core::types::Address,
            receiver: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 85, 82, 120], (token, receiver))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `CancelDecreasePosition` event"]
        pub fn cancel_decrease_position_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, CancelDecreasePositionFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `CancelIncreasePosition` event"]
        pub fn cancel_increase_position_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, CancelIncreasePositionFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `CreateDecreasePosition` event"]
        pub fn create_decrease_position_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, CreateDecreasePositionFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `CreateIncreasePosition` event"]
        pub fn create_increase_position_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, CreateIncreasePositionFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `DecreasePositionReferral` event"]
        pub fn decrease_position_referral_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, DecreasePositionReferralFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ExecuteDecreasePosition` event"]
        pub fn execute_decrease_position_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ExecuteDecreasePositionFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ExecuteIncreasePosition` event"]
        pub fn execute_increase_position_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ExecuteIncreasePositionFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `IncreasePositionReferral` event"]
        pub fn increase_position_referral_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, IncreasePositionReferralFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `SetAdmin` event"]
        pub fn set_admin_filter(&self) -> ethers::contract::builders::Event<M, SetAdminFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `SetDelayValues` event"]
        pub fn set_delay_values_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, SetDelayValuesFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `SetDepositFee` event"]
        pub fn set_deposit_fee_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, SetDepositFeeFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `SetIncreasePositionBufferBps` event"]
        pub fn set_increase_position_buffer_bps_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, SetIncreasePositionBufferBpsFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `SetIsLeverageEnabled` event"]
        pub fn set_is_leverage_enabled_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, SetIsLeverageEnabledFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `SetMaxGlobalSizes` event"]
        pub fn set_max_global_sizes_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, SetMaxGlobalSizesFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `SetMinExecutionFee` event"]
        pub fn set_min_execution_fee_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, SetMinExecutionFeeFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `SetPositionKeeper` event"]
        pub fn set_position_keeper_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, SetPositionKeeperFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `SetReferralStorage` event"]
        pub fn set_referral_storage_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, SetReferralStorageFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `SetRequestKeysStartValues` event"]
        pub fn set_request_keys_start_values_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, SetRequestKeysStartValuesFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `WithdrawFees` event"]
        pub fn withdraw_fees_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, WithdrawFeesFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, b_contractEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for b_contract<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(
        name = "CancelDecreasePosition",
        abi = "CancelDecreasePosition(address,address[],address,uint256,uint256,bool,address,uint256,uint256,uint256,uint256,uint256)"
    )]
    pub struct CancelDecreasePositionFilter {
        #[ethevent(indexed)]
        pub account: ethers::core::types::Address,
        pub path: Vec<ethers::core::types::Address>,
        pub index_token: ethers::core::types::Address,
        pub collateral_delta: ethers::core::types::U256,
        pub size_delta: ethers::core::types::U256,
        pub is_long: bool,
        pub receiver: ethers::core::types::Address,
        pub acceptable_price: ethers::core::types::U256,
        pub min_out: ethers::core::types::U256,
        pub execution_fee: ethers::core::types::U256,
        pub block_gap: ethers::core::types::U256,
        pub time_gap: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(
        name = "CancelIncreasePosition",
        abi = "CancelIncreasePosition(address,address[],address,uint256,uint256,uint256,bool,uint256,uint256,uint256,uint256)"
    )]
    pub struct CancelIncreasePositionFilter {
        #[ethevent(indexed)]
        pub account: ethers::core::types::Address,
        pub path: Vec<ethers::core::types::Address>,
        pub index_token: ethers::core::types::Address,
        pub amount_in: ethers::core::types::U256,
        pub min_out: ethers::core::types::U256,
        pub size_delta: ethers::core::types::U256,
        pub is_long: bool,
        pub acceptable_price: ethers::core::types::U256,
        pub execution_fee: ethers::core::types::U256,
        pub block_gap: ethers::core::types::U256,
        pub time_gap: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(
        name = "CreateDecreasePosition",
        abi = "CreateDecreasePosition(address,address[],address,uint256,uint256,bool,address,uint256,uint256,uint256,uint256,uint256,uint256)"
    )]
    pub struct CreateDecreasePositionFilter {
        #[ethevent(indexed)]
        pub account: ethers::core::types::Address,
        pub path: Vec<ethers::core::types::Address>,
        pub index_token: ethers::core::types::Address,
        pub collateral_delta: ethers::core::types::U256,
        pub size_delta: ethers::core::types::U256,
        pub is_long: bool,
        pub receiver: ethers::core::types::Address,
        pub acceptable_price: ethers::core::types::U256,
        pub min_out: ethers::core::types::U256,
        pub execution_fee: ethers::core::types::U256,
        pub index: ethers::core::types::U256,
        pub block_number: ethers::core::types::U256,
        pub block_time: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(
        name = "CreateIncreasePosition",
        abi = "CreateIncreasePosition(address,address[],address,uint256,uint256,uint256,bool,uint256,uint256,uint256,uint256,uint256,uint256)"
    )]
    pub struct CreateIncreasePositionFilter {
        #[ethevent(indexed)]
        pub account: ethers::core::types::Address,
        pub path: Vec<ethers::core::types::Address>,
        pub index_token: ethers::core::types::Address,
        pub amount_in: ethers::core::types::U256,
        pub min_out: ethers::core::types::U256,
        pub size_delta: ethers::core::types::U256,
        pub is_long: bool,
        pub acceptable_price: ethers::core::types::U256,
        pub execution_fee: ethers::core::types::U256,
        pub index: ethers::core::types::U256,
        pub block_number: ethers::core::types::U256,
        pub block_time: ethers::core::types::U256,
        pub gas_price: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(
        name = "DecreasePositionReferral",
        abi = "DecreasePositionReferral(address,uint256,uint256,bytes32,address)"
    )]
    pub struct DecreasePositionReferralFilter {
        pub account: ethers::core::types::Address,
        pub size_delta: ethers::core::types::U256,
        pub margin_fee_basis_points: ethers::core::types::U256,
        pub referral_code: [u8; 32],
        pub referrer: ethers::core::types::Address,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(
        name = "ExecuteDecreasePosition",
        abi = "ExecuteDecreasePosition(address,address[],address,uint256,uint256,bool,address,uint256,uint256,uint256,uint256,uint256)"
    )]
    pub struct ExecuteDecreasePositionFilter {
        #[ethevent(indexed)]
        pub account: ethers::core::types::Address,
        pub path: Vec<ethers::core::types::Address>,
        pub index_token: ethers::core::types::Address,
        pub collateral_delta: ethers::core::types::U256,
        pub size_delta: ethers::core::types::U256,
        pub is_long: bool,
        pub receiver: ethers::core::types::Address,
        pub acceptable_price: ethers::core::types::U256,
        pub min_out: ethers::core::types::U256,
        pub execution_fee: ethers::core::types::U256,
        pub block_gap: ethers::core::types::U256,
        pub time_gap: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(
        name = "ExecuteIncreasePosition",
        abi = "ExecuteIncreasePosition(address,address[],address,uint256,uint256,uint256,bool,uint256,uint256,uint256,uint256)"
    )]
    pub struct ExecuteIncreasePositionFilter {
        #[ethevent(indexed)]
        pub account: ethers::core::types::Address,
        pub path: Vec<ethers::core::types::Address>,
        pub index_token: ethers::core::types::Address,
        pub amount_in: ethers::core::types::U256,
        pub min_out: ethers::core::types::U256,
        pub size_delta: ethers::core::types::U256,
        pub is_long: bool,
        pub acceptable_price: ethers::core::types::U256,
        pub execution_fee: ethers::core::types::U256,
        pub block_gap: ethers::core::types::U256,
        pub time_gap: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(
        name = "IncreasePositionReferral",
        abi = "IncreasePositionReferral(address,uint256,uint256,bytes32,address)"
    )]
    pub struct IncreasePositionReferralFilter {
        pub account: ethers::core::types::Address,
        pub size_delta: ethers::core::types::U256,
        pub margin_fee_basis_points: ethers::core::types::U256,
        pub referral_code: [u8; 32],
        pub referrer: ethers::core::types::Address,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "SetAdmin", abi = "SetAdmin(address)")]
    pub struct SetAdminFilter {
        pub admin: ethers::core::types::Address,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(
        name = "SetDelayValues",
        abi = "SetDelayValues(uint256,uint256,uint256)"
    )]
    pub struct SetDelayValuesFilter {
        pub min_block_delay_keeper: ethers::core::types::U256,
        pub min_time_delay_public: ethers::core::types::U256,
        pub max_time_delay: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "SetDepositFee", abi = "SetDepositFee(uint256)")]
    pub struct SetDepositFeeFilter {
        pub deposit_fee: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(
        name = "SetIncreasePositionBufferBps",
        abi = "SetIncreasePositionBufferBps(uint256)"
    )]
    pub struct SetIncreasePositionBufferBpsFilter {
        pub increase_position_buffer_bps: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "SetIsLeverageEnabled", abi = "SetIsLeverageEnabled(bool)")]
    pub struct SetIsLeverageEnabledFilter {
        pub is_leverage_enabled: bool,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(
        name = "SetMaxGlobalSizes",
        abi = "SetMaxGlobalSizes(address[],uint256[],uint256[])"
    )]
    pub struct SetMaxGlobalSizesFilter {
        pub tokens: Vec<ethers::core::types::Address>,
        pub long_sizes: Vec<ethers::core::types::U256>,
        pub short_sizes: Vec<ethers::core::types::U256>,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "SetMinExecutionFee", abi = "SetMinExecutionFee(uint256)")]
    pub struct SetMinExecutionFeeFilter {
        pub min_execution_fee: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "SetPositionKeeper", abi = "SetPositionKeeper(address,bool)")]
    pub struct SetPositionKeeperFilter {
        #[ethevent(indexed)]
        pub account: ethers::core::types::Address,
        pub is_active: bool,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "SetReferralStorage", abi = "SetReferralStorage(address)")]
    pub struct SetReferralStorageFilter {
        pub referral_storage: ethers::core::types::Address,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(
        name = "SetRequestKeysStartValues",
        abi = "SetRequestKeysStartValues(uint256,uint256)"
    )]
    pub struct SetRequestKeysStartValuesFilter {
        pub increase_position_request_keys_start: ethers::core::types::U256,
        pub decrease_position_request_keys_start: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "WithdrawFees", abi = "WithdrawFees(address,address,uint256)")]
    pub struct WithdrawFeesFilter {
        pub token: ethers::core::types::Address,
        pub receiver: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum b_contractEvents {
        CancelDecreasePositionFilter(CancelDecreasePositionFilter),
        CancelIncreasePositionFilter(CancelIncreasePositionFilter),
        CreateDecreasePositionFilter(CreateDecreasePositionFilter),
        CreateIncreasePositionFilter(CreateIncreasePositionFilter),
        DecreasePositionReferralFilter(DecreasePositionReferralFilter),
        ExecuteDecreasePositionFilter(ExecuteDecreasePositionFilter),
        ExecuteIncreasePositionFilter(ExecuteIncreasePositionFilter),
        IncreasePositionReferralFilter(IncreasePositionReferralFilter),
        SetAdminFilter(SetAdminFilter),
        SetDelayValuesFilter(SetDelayValuesFilter),
        SetDepositFeeFilter(SetDepositFeeFilter),
        SetIncreasePositionBufferBpsFilter(SetIncreasePositionBufferBpsFilter),
        SetIsLeverageEnabledFilter(SetIsLeverageEnabledFilter),
        SetMaxGlobalSizesFilter(SetMaxGlobalSizesFilter),
        SetMinExecutionFeeFilter(SetMinExecutionFeeFilter),
        SetPositionKeeperFilter(SetPositionKeeperFilter),
        SetReferralStorageFilter(SetReferralStorageFilter),
        SetRequestKeysStartValuesFilter(SetRequestKeysStartValuesFilter),
        WithdrawFeesFilter(WithdrawFeesFilter),
    }
    impl ethers::contract::EthLogDecode for b_contractEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = CancelDecreasePositionFilter::decode_log(log) {
                return Ok(b_contractEvents::CancelDecreasePositionFilter(decoded));
            }
            if let Ok(decoded) = CancelIncreasePositionFilter::decode_log(log) {
                return Ok(b_contractEvents::CancelIncreasePositionFilter(decoded));
            }
            if let Ok(decoded) = CreateDecreasePositionFilter::decode_log(log) {
                return Ok(b_contractEvents::CreateDecreasePositionFilter(decoded));
            }
            if let Ok(decoded) = CreateIncreasePositionFilter::decode_log(log) {
                return Ok(b_contractEvents::CreateIncreasePositionFilter(decoded));
            }
            if let Ok(decoded) = DecreasePositionReferralFilter::decode_log(log) {
                return Ok(b_contractEvents::DecreasePositionReferralFilter(decoded));
            }
            if let Ok(decoded) = ExecuteDecreasePositionFilter::decode_log(log) {
                return Ok(b_contractEvents::ExecuteDecreasePositionFilter(decoded));
            }
            if let Ok(decoded) = ExecuteIncreasePositionFilter::decode_log(log) {
                return Ok(b_contractEvents::ExecuteIncreasePositionFilter(decoded));
            }
            if let Ok(decoded) = IncreasePositionReferralFilter::decode_log(log) {
                return Ok(b_contractEvents::IncreasePositionReferralFilter(decoded));
            }
            if let Ok(decoded) = SetAdminFilter::decode_log(log) {
                return Ok(b_contractEvents::SetAdminFilter(decoded));
            }
            if let Ok(decoded) = SetDelayValuesFilter::decode_log(log) {
                return Ok(b_contractEvents::SetDelayValuesFilter(decoded));
            }
            if let Ok(decoded) = SetDepositFeeFilter::decode_log(log) {
                return Ok(b_contractEvents::SetDepositFeeFilter(decoded));
            }
            if let Ok(decoded) = SetIncreasePositionBufferBpsFilter::decode_log(log) {
                return Ok(b_contractEvents::SetIncreasePositionBufferBpsFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = SetIsLeverageEnabledFilter::decode_log(log) {
                return Ok(b_contractEvents::SetIsLeverageEnabledFilter(decoded));
            }
            if let Ok(decoded) = SetMaxGlobalSizesFilter::decode_log(log) {
                return Ok(b_contractEvents::SetMaxGlobalSizesFilter(decoded));
            }
            if let Ok(decoded) = SetMinExecutionFeeFilter::decode_log(log) {
                return Ok(b_contractEvents::SetMinExecutionFeeFilter(decoded));
            }
            if let Ok(decoded) = SetPositionKeeperFilter::decode_log(log) {
                return Ok(b_contractEvents::SetPositionKeeperFilter(decoded));
            }
            if let Ok(decoded) = SetReferralStorageFilter::decode_log(log) {
                return Ok(b_contractEvents::SetReferralStorageFilter(decoded));
            }
            if let Ok(decoded) = SetRequestKeysStartValuesFilter::decode_log(log) {
                return Ok(b_contractEvents::SetRequestKeysStartValuesFilter(decoded));
            }
            if let Ok(decoded) = WithdrawFeesFilter::decode_log(log) {
                return Ok(b_contractEvents::WithdrawFeesFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for b_contractEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                b_contractEvents::CancelDecreasePositionFilter(element) => element.fmt(f),
                b_contractEvents::CancelIncreasePositionFilter(element) => element.fmt(f),
                b_contractEvents::CreateDecreasePositionFilter(element) => element.fmt(f),
                b_contractEvents::CreateIncreasePositionFilter(element) => element.fmt(f),
                b_contractEvents::DecreasePositionReferralFilter(element) => element.fmt(f),
                b_contractEvents::ExecuteDecreasePositionFilter(element) => element.fmt(f),
                b_contractEvents::ExecuteIncreasePositionFilter(element) => element.fmt(f),
                b_contractEvents::IncreasePositionReferralFilter(element) => element.fmt(f),
                b_contractEvents::SetAdminFilter(element) => element.fmt(f),
                b_contractEvents::SetDelayValuesFilter(element) => element.fmt(f),
                b_contractEvents::SetDepositFeeFilter(element) => element.fmt(f),
                b_contractEvents::SetIncreasePositionBufferBpsFilter(element) => element.fmt(f),
                b_contractEvents::SetIsLeverageEnabledFilter(element) => element.fmt(f),
                b_contractEvents::SetMaxGlobalSizesFilter(element) => element.fmt(f),
                b_contractEvents::SetMinExecutionFeeFilter(element) => element.fmt(f),
                b_contractEvents::SetPositionKeeperFilter(element) => element.fmt(f),
                b_contractEvents::SetReferralStorageFilter(element) => element.fmt(f),
                b_contractEvents::SetRequestKeysStartValuesFilter(element) => element.fmt(f),
                b_contractEvents::WithdrawFeesFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `BASIS_POINTS_DIVISOR` function with signature `BASIS_POINTS_DIVISOR()` and selector `[18, 96, 130, 207]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "BASIS_POINTS_DIVISOR", abi = "BASIS_POINTS_DIVISOR()")]
    pub struct BasisPointsDivisorCall;
    #[doc = "Container type for all input parameters for the `admin` function with signature `admin()` and selector `[248, 81, 164, 64]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "admin", abi = "admin()")]
    pub struct AdminCall;
    #[doc = "Container type for all input parameters for the `approve` function with signature `approve(address,address,uint256)` and selector `[225, 242, 28, 103]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "approve", abi = "approve(address,address,uint256)")]
    pub struct ApproveCall {
        pub token: ethers::core::types::Address,
        pub spender: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `cancelDecreasePosition` function with signature `cancelDecreasePosition(bytes32,address)` and selector `[96, 163, 98, 226]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "cancelDecreasePosition",
        abi = "cancelDecreasePosition(bytes32,address)"
    )]
    pub struct CancelDecreasePositionCall {
        pub key: [u8; 32],
        pub execution_fee_receiver: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `cancelIncreasePosition` function with signature `cancelIncreasePosition(bytes32,address)` and selector `[34, 95, 201, 253]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "cancelIncreasePosition",
        abi = "cancelIncreasePosition(bytes32,address)"
    )]
    pub struct CancelIncreasePositionCall {
        pub key: [u8; 32],
        pub execution_fee_receiver: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `createDecreasePosition` function with signature `createDecreasePosition(address[],address,uint256,uint256,bool,address,uint256,uint256,uint256,bool)` and selector `[231, 13, 210, 252]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "createDecreasePosition",
        abi = "createDecreasePosition(address[],address,uint256,uint256,bool,address,uint256,uint256,uint256,bool)"
    )]
    pub struct CreateDecreasePositionCall {
        pub path: ::std::vec::Vec<ethers::core::types::Address>,
        pub index_token: ethers::core::types::Address,
        pub collateral_delta: ethers::core::types::U256,
        pub size_delta: ethers::core::types::U256,
        pub is_long: bool,
        pub receiver: ethers::core::types::Address,
        pub acceptable_price: ethers::core::types::U256,
        pub min_out: ethers::core::types::U256,
        pub execution_fee: ethers::core::types::U256,
        pub withdraw_eth: bool,
    }
    #[doc = "Container type for all input parameters for the `createIncreasePosition` function with signature `createIncreasePosition(address[],address,uint256,uint256,uint256,bool,uint256,uint256,bytes32)` and selector `[201, 135, 65, 112]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "createIncreasePosition",
        abi = "createIncreasePosition(address[],address,uint256,uint256,uint256,bool,uint256,uint256,bytes32)"
    )]
    pub struct CreateIncreasePositionCall {
        pub path: ::std::vec::Vec<ethers::core::types::Address>,
        pub index_token: ethers::core::types::Address,
        pub amount_in: ethers::core::types::U256,
        pub min_out: ethers::core::types::U256,
        pub size_delta: ethers::core::types::U256,
        pub is_long: bool,
        pub acceptable_price: ethers::core::types::U256,
        pub execution_fee: ethers::core::types::U256,
        pub referral_code: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `createIncreasePositionETH` function with signature `createIncreasePositionETH(address[],address,uint256,uint256,bool,uint256,uint256,bytes32)` and selector `[51, 46, 3, 130]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "createIncreasePositionETH",
        abi = "createIncreasePositionETH(address[],address,uint256,uint256,bool,uint256,uint256,bytes32)"
    )]
    pub struct CreateIncreasePositionETHCall {
        pub path: ::std::vec::Vec<ethers::core::types::Address>,
        pub index_token: ethers::core::types::Address,
        pub min_out: ethers::core::types::U256,
        pub size_delta: ethers::core::types::U256,
        pub is_long: bool,
        pub acceptable_price: ethers::core::types::U256,
        pub execution_fee: ethers::core::types::U256,
        pub referral_code: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `decreasePositionRequestKeys` function with signature `decreasePositionRequestKeys(uint256)` and selector `[66, 120, 85, 95]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "decreasePositionRequestKeys",
        abi = "decreasePositionRequestKeys(uint256)"
    )]
    pub struct DecreasePositionRequestKeysCall(pub ethers::core::types::U256);
    #[doc = "Container type for all input parameters for the `decreasePositionRequestKeysStart` function with signature `decreasePositionRequestKeysStart()` and selector `[27, 202, 140, 240]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "decreasePositionRequestKeysStart",
        abi = "decreasePositionRequestKeysStart()"
    )]
    pub struct DecreasePositionRequestKeysStartCall;
    #[doc = "Container type for all input parameters for the `decreasePositionRequests` function with signature `decreasePositionRequests(bytes32)` and selector `[31, 40, 81, 6]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "decreasePositionRequests",
        abi = "decreasePositionRequests(bytes32)"
    )]
    pub struct DecreasePositionRequestsCall(pub [u8; 32]);
    #[doc = "Container type for all input parameters for the `decreasePositionsIndex` function with signature `decreasePositionsIndex(address)` and selector `[250, 68, 69, 119]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "decreasePositionsIndex",
        abi = "decreasePositionsIndex(address)"
    )]
    pub struct DecreasePositionsIndexCall(pub ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `depositFee` function with signature `depositFee()` and selector `[103, 165, 39, 147]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "depositFee", abi = "depositFee()")]
    pub struct DepositFeeCall;
    #[doc = "Container type for all input parameters for the `executeDecreasePosition` function with signature `executeDecreasePosition(bytes32,address)` and selector `[13, 77, 0, 61]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "executeDecreasePosition",
        abi = "executeDecreasePosition(bytes32,address)"
    )]
    pub struct ExecuteDecreasePositionCall {
        pub key: [u8; 32],
        pub execution_fee_receiver: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `executeDecreasePositions` function with signature `executeDecreasePositions(uint256,address)` and selector `[243, 136, 61, 139]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "executeDecreasePositions",
        abi = "executeDecreasePositions(uint256,address)"
    )]
    pub struct ExecuteDecreasePositionsCall {
        pub end_index: ethers::core::types::U256,
        pub execution_fee_receiver: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `executeIncreasePosition` function with signature `executeIncreasePosition(bytes32,address)` and selector `[39, 180, 44, 15]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "executeIncreasePosition",
        abi = "executeIncreasePosition(bytes32,address)"
    )]
    pub struct ExecuteIncreasePositionCall {
        pub key: [u8; 32],
        pub execution_fee_receiver: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `executeIncreasePositions` function with signature `executeIncreasePositions(uint256,address)` and selector `[154, 32, 129, 0]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "executeIncreasePositions",
        abi = "executeIncreasePositions(uint256,address)"
    )]
    pub struct ExecuteIncreasePositionsCall {
        pub end_index: ethers::core::types::U256,
        pub execution_fee_receiver: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `feeReserves` function with signature `feeReserves(address)` and selector `[28, 233, 203, 143]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "feeReserves", abi = "feeReserves(address)")]
    pub struct FeeReservesCall(pub ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `getDecreasePositionRequestPath` function with signature `getDecreasePositionRequestPath(bytes32)` and selector `[93, 92, 34, 232]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "getDecreasePositionRequestPath",
        abi = "getDecreasePositionRequestPath(bytes32)"
    )]
    pub struct GetDecreasePositionRequestPathCall {
        pub key: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `getIncreasePositionRequestPath` function with signature `getIncreasePositionRequestPath(bytes32)` and selector `[149, 233, 187, 215]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "getIncreasePositionRequestPath",
        abi = "getIncreasePositionRequestPath(bytes32)"
    )]
    pub struct GetIncreasePositionRequestPathCall {
        pub key: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `getRequestKey` function with signature `getRequestKey(address,uint256)` and selector `[98, 248, 163, 254]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getRequestKey", abi = "getRequestKey(address,uint256)")]
    pub struct GetRequestKeyCall {
        pub account: ethers::core::types::Address,
        pub index: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `getRequestQueueLengths` function with signature `getRequestQueueLengths()` and selector `[242, 206, 166, 165]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getRequestQueueLengths", abi = "getRequestQueueLengths()")]
    pub struct GetRequestQueueLengthsCall;
    #[doc = "Container type for all input parameters for the `gov` function with signature `gov()` and selector `[18, 212, 58, 81]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "gov", abi = "gov()")]
    pub struct GovCall;
    #[doc = "Container type for all input parameters for the `increasePositionBufferBps` function with signature `increasePositionBufferBps()` and selector `[152, 209, 224, 58]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "increasePositionBufferBps",
        abi = "increasePositionBufferBps()"
    )]
    pub struct IncreasePositionBufferBpsCall;
    #[doc = "Container type for all input parameters for the `increasePositionRequestKeys` function with signature `increasePositionRequestKeys(uint256)` and selector `[4, 34, 89, 84]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "increasePositionRequestKeys",
        abi = "increasePositionRequestKeys(uint256)"
    )]
    pub struct IncreasePositionRequestKeysCall(pub ethers::core::types::U256);
    #[doc = "Container type for all input parameters for the `increasePositionRequestKeysStart` function with signature `increasePositionRequestKeysStart()` and selector `[155, 87, 134, 32]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "increasePositionRequestKeysStart",
        abi = "increasePositionRequestKeysStart()"
    )]
    pub struct IncreasePositionRequestKeysStartCall;
    #[doc = "Container type for all input parameters for the `increasePositionRequests` function with signature `increasePositionRequests(bytes32)` and selector `[250, 249, 144, 243]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "increasePositionRequests",
        abi = "increasePositionRequests(bytes32)"
    )]
    pub struct IncreasePositionRequestsCall(pub [u8; 32]);
    #[doc = "Container type for all input parameters for the `increasePositionsIndex` function with signature `increasePositionsIndex(address)` and selector `[99, 52, 81, 222]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "increasePositionsIndex",
        abi = "increasePositionsIndex(address)"
    )]
    pub struct IncreasePositionsIndexCall(pub ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `isLeverageEnabled` function with signature `isLeverageEnabled()` and selector `[62, 114, 162, 98]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "isLeverageEnabled", abi = "isLeverageEnabled()")]
    pub struct IsLeverageEnabledCall;
    #[doc = "Container type for all input parameters for the `isPositionKeeper` function with signature `isPositionKeeper(address)` and selector `[54, 235, 164, 138]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "isPositionKeeper", abi = "isPositionKeeper(address)")]
    pub struct IsPositionKeeperCall(pub ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `maxGlobalLongSizes` function with signature `maxGlobalLongSizes(address)` and selector `[16, 69, 199, 78]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "maxGlobalLongSizes", abi = "maxGlobalLongSizes(address)")]
    pub struct MaxGlobalLongSizesCall(pub ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `maxGlobalShortSizes` function with signature `maxGlobalShortSizes(address)` and selector `[150, 152, 210, 90]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "maxGlobalShortSizes", abi = "maxGlobalShortSizes(address)")]
    pub struct MaxGlobalShortSizesCall(pub ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `maxTimeDelay` function with signature `maxTimeDelay()` and selector `[203, 2, 105, 201]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "maxTimeDelay", abi = "maxTimeDelay()")]
    pub struct MaxTimeDelayCall;
    #[doc = "Container type for all input parameters for the `minBlockDelayKeeper` function with signature `minBlockDelayKeeper()` and selector `[88, 65, 252, 170]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "minBlockDelayKeeper", abi = "minBlockDelayKeeper()")]
    pub struct MinBlockDelayKeeperCall;
    #[doc = "Container type for all input parameters for the `minExecutionFee` function with signature `minExecutionFee()` and selector `[99, 174, 33, 3]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "minExecutionFee", abi = "minExecutionFee()")]
    pub struct MinExecutionFeeCall;
    #[doc = "Container type for all input parameters for the `minTimeDelayPublic` function with signature `minTimeDelayPublic()` and selector `[58, 42, 128, 199]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "minTimeDelayPublic", abi = "minTimeDelayPublic()")]
    pub struct MinTimeDelayPublicCall;
    #[doc = "Container type for all input parameters for the `referralStorage` function with signature `referralStorage()` and selector `[0, 108, 195, 94]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "referralStorage", abi = "referralStorage()")]
    pub struct ReferralStorageCall;
    #[doc = "Container type for all input parameters for the `router` function with signature `router()` and selector `[248, 135, 234, 64]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "router", abi = "router()")]
    pub struct RouterCall;
    #[doc = "Container type for all input parameters for the `sendValue` function with signature `sendValue(address,uint256)` and selector `[36, 160, 132, 223]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "sendValue", abi = "sendValue(address,uint256)")]
    pub struct SendValueCall {
        pub receiver: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setAdmin` function with signature `setAdmin(address)` and selector `[112, 75, 108, 2]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setAdmin", abi = "setAdmin(address)")]
    pub struct SetAdminCall {
        pub admin: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setDelayValues` function with signature `setDelayValues(uint256,uint256,uint256)` and selector `[64, 103, 177, 50]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "setDelayValues",
        abi = "setDelayValues(uint256,uint256,uint256)"
    )]
    pub struct SetDelayValuesCall {
        pub min_block_delay_keeper: ethers::core::types::U256,
        pub min_time_delay_public: ethers::core::types::U256,
        pub max_time_delay: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setDepositFee` function with signature `setDepositFee(uint256)` and selector `[73, 10, 226, 16]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setDepositFee", abi = "setDepositFee(uint256)")]
    pub struct SetDepositFeeCall {
        pub deposit_fee: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setGov` function with signature `setGov(address)` and selector `[207, 173, 87, 162]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setGov", abi = "setGov(address)")]
    pub struct SetGovCall {
        pub gov: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setIncreasePositionBufferBps` function with signature `setIncreasePositionBufferBps(uint256)` and selector `[35, 59, 254, 59]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "setIncreasePositionBufferBps",
        abi = "setIncreasePositionBufferBps(uint256)"
    )]
    pub struct SetIncreasePositionBufferBpsCall {
        pub increase_position_buffer_bps: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setIsLeverageEnabled` function with signature `setIsLeverageEnabled(bool)` and selector `[124, 46, 185, 247]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setIsLeverageEnabled", abi = "setIsLeverageEnabled(bool)")]
    pub struct SetIsLeverageEnabledCall {
        pub is_leverage_enabled: bool,
    }
    #[doc = "Container type for all input parameters for the `setMaxGlobalSizes` function with signature `setMaxGlobalSizes(address[],uint256[],uint256[])` and selector `[239, 18, 198, 126]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "setMaxGlobalSizes",
        abi = "setMaxGlobalSizes(address[],uint256[],uint256[])"
    )]
    pub struct SetMaxGlobalSizesCall {
        pub tokens: ::std::vec::Vec<ethers::core::types::Address>,
        pub long_sizes: ::std::vec::Vec<ethers::core::types::U256>,
        pub short_sizes: ::std::vec::Vec<ethers::core::types::U256>,
    }
    #[doc = "Container type for all input parameters for the `setMinExecutionFee` function with signature `setMinExecutionFee(uint256)` and selector `[252, 44, 238, 98]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setMinExecutionFee", abi = "setMinExecutionFee(uint256)")]
    pub struct SetMinExecutionFeeCall {
        pub min_execution_fee: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setPositionKeeper` function with signature `setPositionKeeper(address,bool)` and selector `[52, 34, 234, 209]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setPositionKeeper", abi = "setPositionKeeper(address,bool)")]
    pub struct SetPositionKeeperCall {
        pub account: ethers::core::types::Address,
        pub is_active: bool,
    }
    #[doc = "Container type for all input parameters for the `setReferralStorage` function with signature `setReferralStorage(address)` and selector `[174, 77, 127, 154]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setReferralStorage", abi = "setReferralStorage(address)")]
    pub struct SetReferralStorageCall {
        pub referral_storage: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setRequestKeysStartValues` function with signature `setRequestKeysStartValues(uint256,uint256)` and selector `[48, 138, 168, 31]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "setRequestKeysStartValues",
        abi = "setRequestKeysStartValues(uint256,uint256)"
    )]
    pub struct SetRequestKeysStartValuesCall {
        pub increase_position_request_keys_start: ethers::core::types::U256,
        pub decrease_position_request_keys_start: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `vault` function with signature `vault()` and selector `[251, 250, 119, 207]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "vault", abi = "vault()")]
    pub struct VaultCall;
    #[doc = "Container type for all input parameters for the `weth` function with signature `weth()` and selector `[63, 200, 206, 243]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "weth", abi = "weth()")]
    pub struct WethCall;
    #[doc = "Container type for all input parameters for the `withdrawFees` function with signature `withdrawFees(address,address)` and selector `[242, 85, 82, 120]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "withdrawFees", abi = "withdrawFees(address,address)")]
    pub struct WithdrawFeesCall {
        pub token: ethers::core::types::Address,
        pub receiver: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum b_contractCalls {
        BasisPointsDivisor(BasisPointsDivisorCall),
        Admin(AdminCall),
        Approve(ApproveCall),
        CancelDecreasePosition(CancelDecreasePositionCall),
        CancelIncreasePosition(CancelIncreasePositionCall),
        CreateDecreasePosition(CreateDecreasePositionCall),
        CreateIncreasePosition(CreateIncreasePositionCall),
        CreateIncreasePositionETH(CreateIncreasePositionETHCall),
        DecreasePositionRequestKeys(DecreasePositionRequestKeysCall),
        DecreasePositionRequestKeysStart(DecreasePositionRequestKeysStartCall),
        DecreasePositionRequests(DecreasePositionRequestsCall),
        DecreasePositionsIndex(DecreasePositionsIndexCall),
        DepositFee(DepositFeeCall),
        ExecuteDecreasePosition(ExecuteDecreasePositionCall),
        ExecuteDecreasePositions(ExecuteDecreasePositionsCall),
        ExecuteIncreasePosition(ExecuteIncreasePositionCall),
        ExecuteIncreasePositions(ExecuteIncreasePositionsCall),
        FeeReserves(FeeReservesCall),
        GetDecreasePositionRequestPath(GetDecreasePositionRequestPathCall),
        GetIncreasePositionRequestPath(GetIncreasePositionRequestPathCall),
        GetRequestKey(GetRequestKeyCall),
        GetRequestQueueLengths(GetRequestQueueLengthsCall),
        Gov(GovCall),
        IncreasePositionBufferBps(IncreasePositionBufferBpsCall),
        IncreasePositionRequestKeys(IncreasePositionRequestKeysCall),
        IncreasePositionRequestKeysStart(IncreasePositionRequestKeysStartCall),
        IncreasePositionRequests(IncreasePositionRequestsCall),
        IncreasePositionsIndex(IncreasePositionsIndexCall),
        IsLeverageEnabled(IsLeverageEnabledCall),
        IsPositionKeeper(IsPositionKeeperCall),
        MaxGlobalLongSizes(MaxGlobalLongSizesCall),
        MaxGlobalShortSizes(MaxGlobalShortSizesCall),
        MaxTimeDelay(MaxTimeDelayCall),
        MinBlockDelayKeeper(MinBlockDelayKeeperCall),
        MinExecutionFee(MinExecutionFeeCall),
        MinTimeDelayPublic(MinTimeDelayPublicCall),
        ReferralStorage(ReferralStorageCall),
        Router(RouterCall),
        SendValue(SendValueCall),
        SetAdmin(SetAdminCall),
        SetDelayValues(SetDelayValuesCall),
        SetDepositFee(SetDepositFeeCall),
        SetGov(SetGovCall),
        SetIncreasePositionBufferBps(SetIncreasePositionBufferBpsCall),
        SetIsLeverageEnabled(SetIsLeverageEnabledCall),
        SetMaxGlobalSizes(SetMaxGlobalSizesCall),
        SetMinExecutionFee(SetMinExecutionFeeCall),
        SetPositionKeeper(SetPositionKeeperCall),
        SetReferralStorage(SetReferralStorageCall),
        SetRequestKeysStartValues(SetRequestKeysStartValuesCall),
        Vault(VaultCall),
        Weth(WethCall),
        WithdrawFees(WithdrawFeesCall),
    }
    impl ethers::core::abi::AbiDecode for b_contractCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <BasisPointsDivisorCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(b_contractCalls::BasisPointsDivisor(decoded));
            }
            if let Ok(decoded) = <AdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(b_contractCalls::Admin(decoded));
            }
            if let Ok(decoded) =
                <ApproveCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(b_contractCalls::Approve(decoded));
            }
            if let Ok(decoded) =
                <CancelDecreasePositionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(b_contractCalls::CancelDecreasePosition(decoded));
            }
            if let Ok(decoded) =
                <CancelIncreasePositionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(b_contractCalls::CancelIncreasePosition(decoded));
            }
            if let Ok(decoded) =
                <CreateDecreasePositionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(b_contractCalls::CreateDecreasePosition(decoded));
            }
            if let Ok(decoded) =
                <CreateIncreasePositionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(b_contractCalls::CreateIncreasePosition(decoded));
            }
            if let Ok(decoded) =
                <CreateIncreasePositionETHCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(b_contractCalls::CreateIncreasePositionETH(decoded));
            }
            if let Ok(decoded) =
                <DecreasePositionRequestKeysCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(b_contractCalls::DecreasePositionRequestKeys(decoded));
            }
            if let Ok(decoded) =
                <DecreasePositionRequestKeysStartCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(b_contractCalls::DecreasePositionRequestKeysStart(decoded));
            }
            if let Ok(decoded) =
                <DecreasePositionRequestsCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(b_contractCalls::DecreasePositionRequests(decoded));
            }
            if let Ok(decoded) =
                <DecreasePositionsIndexCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(b_contractCalls::DecreasePositionsIndex(decoded));
            }
            if let Ok(decoded) =
                <DepositFeeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(b_contractCalls::DepositFee(decoded));
            }
            if let Ok(decoded) =
                <ExecuteDecreasePositionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(b_contractCalls::ExecuteDecreasePosition(decoded));
            }
            if let Ok(decoded) =
                <ExecuteDecreasePositionsCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(b_contractCalls::ExecuteDecreasePositions(decoded));
            }
            if let Ok(decoded) =
                <ExecuteIncreasePositionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(b_contractCalls::ExecuteIncreasePosition(decoded));
            }
            if let Ok(decoded) =
                <ExecuteIncreasePositionsCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(b_contractCalls::ExecuteIncreasePositions(decoded));
            }
            if let Ok(decoded) =
                <FeeReservesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(b_contractCalls::FeeReserves(decoded));
            }
            if let Ok(decoded) =
                <GetDecreasePositionRequestPathCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(b_contractCalls::GetDecreasePositionRequestPath(decoded));
            }
            if let Ok(decoded) =
                <GetIncreasePositionRequestPathCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(b_contractCalls::GetIncreasePositionRequestPath(decoded));
            }
            if let Ok(decoded) =
                <GetRequestKeyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(b_contractCalls::GetRequestKey(decoded));
            }
            if let Ok(decoded) =
                <GetRequestQueueLengthsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(b_contractCalls::GetRequestQueueLengths(decoded));
            }
            if let Ok(decoded) = <GovCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(b_contractCalls::Gov(decoded));
            }
            if let Ok(decoded) =
                <IncreasePositionBufferBpsCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(b_contractCalls::IncreasePositionBufferBps(decoded));
            }
            if let Ok(decoded) =
                <IncreasePositionRequestKeysCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(b_contractCalls::IncreasePositionRequestKeys(decoded));
            }
            if let Ok(decoded) =
                <IncreasePositionRequestKeysStartCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(b_contractCalls::IncreasePositionRequestKeysStart(decoded));
            }
            if let Ok(decoded) =
                <IncreasePositionRequestsCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(b_contractCalls::IncreasePositionRequests(decoded));
            }
            if let Ok(decoded) =
                <IncreasePositionsIndexCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(b_contractCalls::IncreasePositionsIndex(decoded));
            }
            if let Ok(decoded) =
                <IsLeverageEnabledCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(b_contractCalls::IsLeverageEnabled(decoded));
            }
            if let Ok(decoded) =
                <IsPositionKeeperCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(b_contractCalls::IsPositionKeeper(decoded));
            }
            if let Ok(decoded) =
                <MaxGlobalLongSizesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(b_contractCalls::MaxGlobalLongSizes(decoded));
            }
            if let Ok(decoded) =
                <MaxGlobalShortSizesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(b_contractCalls::MaxGlobalShortSizes(decoded));
            }
            if let Ok(decoded) =
                <MaxTimeDelayCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(b_contractCalls::MaxTimeDelay(decoded));
            }
            if let Ok(decoded) =
                <MinBlockDelayKeeperCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(b_contractCalls::MinBlockDelayKeeper(decoded));
            }
            if let Ok(decoded) =
                <MinExecutionFeeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(b_contractCalls::MinExecutionFee(decoded));
            }
            if let Ok(decoded) =
                <MinTimeDelayPublicCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(b_contractCalls::MinTimeDelayPublic(decoded));
            }
            if let Ok(decoded) =
                <ReferralStorageCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(b_contractCalls::ReferralStorage(decoded));
            }
            if let Ok(decoded) = <RouterCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(b_contractCalls::Router(decoded));
            }
            if let Ok(decoded) =
                <SendValueCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(b_contractCalls::SendValue(decoded));
            }
            if let Ok(decoded) =
                <SetAdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(b_contractCalls::SetAdmin(decoded));
            }
            if let Ok(decoded) =
                <SetDelayValuesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(b_contractCalls::SetDelayValues(decoded));
            }
            if let Ok(decoded) =
                <SetDepositFeeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(b_contractCalls::SetDepositFee(decoded));
            }
            if let Ok(decoded) = <SetGovCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(b_contractCalls::SetGov(decoded));
            }
            if let Ok(decoded) =
                <SetIncreasePositionBufferBpsCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(b_contractCalls::SetIncreasePositionBufferBps(decoded));
            }
            if let Ok(decoded) =
                <SetIsLeverageEnabledCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(b_contractCalls::SetIsLeverageEnabled(decoded));
            }
            if let Ok(decoded) =
                <SetMaxGlobalSizesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(b_contractCalls::SetMaxGlobalSizes(decoded));
            }
            if let Ok(decoded) =
                <SetMinExecutionFeeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(b_contractCalls::SetMinExecutionFee(decoded));
            }
            if let Ok(decoded) =
                <SetPositionKeeperCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(b_contractCalls::SetPositionKeeper(decoded));
            }
            if let Ok(decoded) =
                <SetReferralStorageCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(b_contractCalls::SetReferralStorage(decoded));
            }
            if let Ok(decoded) =
                <SetRequestKeysStartValuesCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(b_contractCalls::SetRequestKeysStartValues(decoded));
            }
            if let Ok(decoded) = <VaultCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(b_contractCalls::Vault(decoded));
            }
            if let Ok(decoded) = <WethCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(b_contractCalls::Weth(decoded));
            }
            if let Ok(decoded) =
                <WithdrawFeesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(b_contractCalls::WithdrawFees(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for b_contractCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                b_contractCalls::BasisPointsDivisor(element) => element.encode(),
                b_contractCalls::Admin(element) => element.encode(),
                b_contractCalls::Approve(element) => element.encode(),
                b_contractCalls::CancelDecreasePosition(element) => element.encode(),
                b_contractCalls::CancelIncreasePosition(element) => element.encode(),
                b_contractCalls::CreateDecreasePosition(element) => element.encode(),
                b_contractCalls::CreateIncreasePosition(element) => element.encode(),
                b_contractCalls::CreateIncreasePositionETH(element) => element.encode(),
                b_contractCalls::DecreasePositionRequestKeys(element) => element.encode(),
                b_contractCalls::DecreasePositionRequestKeysStart(element) => element.encode(),
                b_contractCalls::DecreasePositionRequests(element) => element.encode(),
                b_contractCalls::DecreasePositionsIndex(element) => element.encode(),
                b_contractCalls::DepositFee(element) => element.encode(),
                b_contractCalls::ExecuteDecreasePosition(element) => element.encode(),
                b_contractCalls::ExecuteDecreasePositions(element) => element.encode(),
                b_contractCalls::ExecuteIncreasePosition(element) => element.encode(),
                b_contractCalls::ExecuteIncreasePositions(element) => element.encode(),
                b_contractCalls::FeeReserves(element) => element.encode(),
                b_contractCalls::GetDecreasePositionRequestPath(element) => element.encode(),
                b_contractCalls::GetIncreasePositionRequestPath(element) => element.encode(),
                b_contractCalls::GetRequestKey(element) => element.encode(),
                b_contractCalls::GetRequestQueueLengths(element) => element.encode(),
                b_contractCalls::Gov(element) => element.encode(),
                b_contractCalls::IncreasePositionBufferBps(element) => element.encode(),
                b_contractCalls::IncreasePositionRequestKeys(element) => element.encode(),
                b_contractCalls::IncreasePositionRequestKeysStart(element) => element.encode(),
                b_contractCalls::IncreasePositionRequests(element) => element.encode(),
                b_contractCalls::IncreasePositionsIndex(element) => element.encode(),
                b_contractCalls::IsLeverageEnabled(element) => element.encode(),
                b_contractCalls::IsPositionKeeper(element) => element.encode(),
                b_contractCalls::MaxGlobalLongSizes(element) => element.encode(),
                b_contractCalls::MaxGlobalShortSizes(element) => element.encode(),
                b_contractCalls::MaxTimeDelay(element) => element.encode(),
                b_contractCalls::MinBlockDelayKeeper(element) => element.encode(),
                b_contractCalls::MinExecutionFee(element) => element.encode(),
                b_contractCalls::MinTimeDelayPublic(element) => element.encode(),
                b_contractCalls::ReferralStorage(element) => element.encode(),
                b_contractCalls::Router(element) => element.encode(),
                b_contractCalls::SendValue(element) => element.encode(),
                b_contractCalls::SetAdmin(element) => element.encode(),
                b_contractCalls::SetDelayValues(element) => element.encode(),
                b_contractCalls::SetDepositFee(element) => element.encode(),
                b_contractCalls::SetGov(element) => element.encode(),
                b_contractCalls::SetIncreasePositionBufferBps(element) => element.encode(),
                b_contractCalls::SetIsLeverageEnabled(element) => element.encode(),
                b_contractCalls::SetMaxGlobalSizes(element) => element.encode(),
                b_contractCalls::SetMinExecutionFee(element) => element.encode(),
                b_contractCalls::SetPositionKeeper(element) => element.encode(),
                b_contractCalls::SetReferralStorage(element) => element.encode(),
                b_contractCalls::SetRequestKeysStartValues(element) => element.encode(),
                b_contractCalls::Vault(element) => element.encode(),
                b_contractCalls::Weth(element) => element.encode(),
                b_contractCalls::WithdrawFees(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for b_contractCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                b_contractCalls::BasisPointsDivisor(element) => element.fmt(f),
                b_contractCalls::Admin(element) => element.fmt(f),
                b_contractCalls::Approve(element) => element.fmt(f),
                b_contractCalls::CancelDecreasePosition(element) => element.fmt(f),
                b_contractCalls::CancelIncreasePosition(element) => element.fmt(f),
                b_contractCalls::CreateDecreasePosition(element) => element.fmt(f),
                b_contractCalls::CreateIncreasePosition(element) => element.fmt(f),
                b_contractCalls::CreateIncreasePositionETH(element) => element.fmt(f),
                b_contractCalls::DecreasePositionRequestKeys(element) => element.fmt(f),
                b_contractCalls::DecreasePositionRequestKeysStart(element) => element.fmt(f),
                b_contractCalls::DecreasePositionRequests(element) => element.fmt(f),
                b_contractCalls::DecreasePositionsIndex(element) => element.fmt(f),
                b_contractCalls::DepositFee(element) => element.fmt(f),
                b_contractCalls::ExecuteDecreasePosition(element) => element.fmt(f),
                b_contractCalls::ExecuteDecreasePositions(element) => element.fmt(f),
                b_contractCalls::ExecuteIncreasePosition(element) => element.fmt(f),
                b_contractCalls::ExecuteIncreasePositions(element) => element.fmt(f),
                b_contractCalls::FeeReserves(element) => element.fmt(f),
                b_contractCalls::GetDecreasePositionRequestPath(element) => element.fmt(f),
                b_contractCalls::GetIncreasePositionRequestPath(element) => element.fmt(f),
                b_contractCalls::GetRequestKey(element) => element.fmt(f),
                b_contractCalls::GetRequestQueueLengths(element) => element.fmt(f),
                b_contractCalls::Gov(element) => element.fmt(f),
                b_contractCalls::IncreasePositionBufferBps(element) => element.fmt(f),
                b_contractCalls::IncreasePositionRequestKeys(element) => element.fmt(f),
                b_contractCalls::IncreasePositionRequestKeysStart(element) => element.fmt(f),
                b_contractCalls::IncreasePositionRequests(element) => element.fmt(f),
                b_contractCalls::IncreasePositionsIndex(element) => element.fmt(f),
                b_contractCalls::IsLeverageEnabled(element) => element.fmt(f),
                b_contractCalls::IsPositionKeeper(element) => element.fmt(f),
                b_contractCalls::MaxGlobalLongSizes(element) => element.fmt(f),
                b_contractCalls::MaxGlobalShortSizes(element) => element.fmt(f),
                b_contractCalls::MaxTimeDelay(element) => element.fmt(f),
                b_contractCalls::MinBlockDelayKeeper(element) => element.fmt(f),
                b_contractCalls::MinExecutionFee(element) => element.fmt(f),
                b_contractCalls::MinTimeDelayPublic(element) => element.fmt(f),
                b_contractCalls::ReferralStorage(element) => element.fmt(f),
                b_contractCalls::Router(element) => element.fmt(f),
                b_contractCalls::SendValue(element) => element.fmt(f),
                b_contractCalls::SetAdmin(element) => element.fmt(f),
                b_contractCalls::SetDelayValues(element) => element.fmt(f),
                b_contractCalls::SetDepositFee(element) => element.fmt(f),
                b_contractCalls::SetGov(element) => element.fmt(f),
                b_contractCalls::SetIncreasePositionBufferBps(element) => element.fmt(f),
                b_contractCalls::SetIsLeverageEnabled(element) => element.fmt(f),
                b_contractCalls::SetMaxGlobalSizes(element) => element.fmt(f),
                b_contractCalls::SetMinExecutionFee(element) => element.fmt(f),
                b_contractCalls::SetPositionKeeper(element) => element.fmt(f),
                b_contractCalls::SetReferralStorage(element) => element.fmt(f),
                b_contractCalls::SetRequestKeysStartValues(element) => element.fmt(f),
                b_contractCalls::Vault(element) => element.fmt(f),
                b_contractCalls::Weth(element) => element.fmt(f),
                b_contractCalls::WithdrawFees(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<BasisPointsDivisorCall> for b_contractCalls {
        fn from(var: BasisPointsDivisorCall) -> Self {
            b_contractCalls::BasisPointsDivisor(var)
        }
    }
    impl ::std::convert::From<AdminCall> for b_contractCalls {
        fn from(var: AdminCall) -> Self {
            b_contractCalls::Admin(var)
        }
    }
    impl ::std::convert::From<ApproveCall> for b_contractCalls {
        fn from(var: ApproveCall) -> Self {
            b_contractCalls::Approve(var)
        }
    }
    impl ::std::convert::From<CancelDecreasePositionCall> for b_contractCalls {
        fn from(var: CancelDecreasePositionCall) -> Self {
            b_contractCalls::CancelDecreasePosition(var)
        }
    }
    impl ::std::convert::From<CancelIncreasePositionCall> for b_contractCalls {
        fn from(var: CancelIncreasePositionCall) -> Self {
            b_contractCalls::CancelIncreasePosition(var)
        }
    }
    impl ::std::convert::From<CreateDecreasePositionCall> for b_contractCalls {
        fn from(var: CreateDecreasePositionCall) -> Self {
            b_contractCalls::CreateDecreasePosition(var)
        }
    }
    impl ::std::convert::From<CreateIncreasePositionCall> for b_contractCalls {
        fn from(var: CreateIncreasePositionCall) -> Self {
            b_contractCalls::CreateIncreasePosition(var)
        }
    }
    impl ::std::convert::From<CreateIncreasePositionETHCall> for b_contractCalls {
        fn from(var: CreateIncreasePositionETHCall) -> Self {
            b_contractCalls::CreateIncreasePositionETH(var)
        }
    }
    impl ::std::convert::From<DecreasePositionRequestKeysCall> for b_contractCalls {
        fn from(var: DecreasePositionRequestKeysCall) -> Self {
            b_contractCalls::DecreasePositionRequestKeys(var)
        }
    }
    impl ::std::convert::From<DecreasePositionRequestKeysStartCall> for b_contractCalls {
        fn from(var: DecreasePositionRequestKeysStartCall) -> Self {
            b_contractCalls::DecreasePositionRequestKeysStart(var)
        }
    }
    impl ::std::convert::From<DecreasePositionRequestsCall> for b_contractCalls {
        fn from(var: DecreasePositionRequestsCall) -> Self {
            b_contractCalls::DecreasePositionRequests(var)
        }
    }
    impl ::std::convert::From<DecreasePositionsIndexCall> for b_contractCalls {
        fn from(var: DecreasePositionsIndexCall) -> Self {
            b_contractCalls::DecreasePositionsIndex(var)
        }
    }
    impl ::std::convert::From<DepositFeeCall> for b_contractCalls {
        fn from(var: DepositFeeCall) -> Self {
            b_contractCalls::DepositFee(var)
        }
    }
    impl ::std::convert::From<ExecuteDecreasePositionCall> for b_contractCalls {
        fn from(var: ExecuteDecreasePositionCall) -> Self {
            b_contractCalls::ExecuteDecreasePosition(var)
        }
    }
    impl ::std::convert::From<ExecuteDecreasePositionsCall> for b_contractCalls {
        fn from(var: ExecuteDecreasePositionsCall) -> Self {
            b_contractCalls::ExecuteDecreasePositions(var)
        }
    }
    impl ::std::convert::From<ExecuteIncreasePositionCall> for b_contractCalls {
        fn from(var: ExecuteIncreasePositionCall) -> Self {
            b_contractCalls::ExecuteIncreasePosition(var)
        }
    }
    impl ::std::convert::From<ExecuteIncreasePositionsCall> for b_contractCalls {
        fn from(var: ExecuteIncreasePositionsCall) -> Self {
            b_contractCalls::ExecuteIncreasePositions(var)
        }
    }
    impl ::std::convert::From<FeeReservesCall> for b_contractCalls {
        fn from(var: FeeReservesCall) -> Self {
            b_contractCalls::FeeReserves(var)
        }
    }
    impl ::std::convert::From<GetDecreasePositionRequestPathCall> for b_contractCalls {
        fn from(var: GetDecreasePositionRequestPathCall) -> Self {
            b_contractCalls::GetDecreasePositionRequestPath(var)
        }
    }
    impl ::std::convert::From<GetIncreasePositionRequestPathCall> for b_contractCalls {
        fn from(var: GetIncreasePositionRequestPathCall) -> Self {
            b_contractCalls::GetIncreasePositionRequestPath(var)
        }
    }
    impl ::std::convert::From<GetRequestKeyCall> for b_contractCalls {
        fn from(var: GetRequestKeyCall) -> Self {
            b_contractCalls::GetRequestKey(var)
        }
    }
    impl ::std::convert::From<GetRequestQueueLengthsCall> for b_contractCalls {
        fn from(var: GetRequestQueueLengthsCall) -> Self {
            b_contractCalls::GetRequestQueueLengths(var)
        }
    }
    impl ::std::convert::From<GovCall> for b_contractCalls {
        fn from(var: GovCall) -> Self {
            b_contractCalls::Gov(var)
        }
    }
    impl ::std::convert::From<IncreasePositionBufferBpsCall> for b_contractCalls {
        fn from(var: IncreasePositionBufferBpsCall) -> Self {
            b_contractCalls::IncreasePositionBufferBps(var)
        }
    }
    impl ::std::convert::From<IncreasePositionRequestKeysCall> for b_contractCalls {
        fn from(var: IncreasePositionRequestKeysCall) -> Self {
            b_contractCalls::IncreasePositionRequestKeys(var)
        }
    }
    impl ::std::convert::From<IncreasePositionRequestKeysStartCall> for b_contractCalls {
        fn from(var: IncreasePositionRequestKeysStartCall) -> Self {
            b_contractCalls::IncreasePositionRequestKeysStart(var)
        }
    }
    impl ::std::convert::From<IncreasePositionRequestsCall> for b_contractCalls {
        fn from(var: IncreasePositionRequestsCall) -> Self {
            b_contractCalls::IncreasePositionRequests(var)
        }
    }
    impl ::std::convert::From<IncreasePositionsIndexCall> for b_contractCalls {
        fn from(var: IncreasePositionsIndexCall) -> Self {
            b_contractCalls::IncreasePositionsIndex(var)
        }
    }
    impl ::std::convert::From<IsLeverageEnabledCall> for b_contractCalls {
        fn from(var: IsLeverageEnabledCall) -> Self {
            b_contractCalls::IsLeverageEnabled(var)
        }
    }
    impl ::std::convert::From<IsPositionKeeperCall> for b_contractCalls {
        fn from(var: IsPositionKeeperCall) -> Self {
            b_contractCalls::IsPositionKeeper(var)
        }
    }
    impl ::std::convert::From<MaxGlobalLongSizesCall> for b_contractCalls {
        fn from(var: MaxGlobalLongSizesCall) -> Self {
            b_contractCalls::MaxGlobalLongSizes(var)
        }
    }
    impl ::std::convert::From<MaxGlobalShortSizesCall> for b_contractCalls {
        fn from(var: MaxGlobalShortSizesCall) -> Self {
            b_contractCalls::MaxGlobalShortSizes(var)
        }
    }
    impl ::std::convert::From<MaxTimeDelayCall> for b_contractCalls {
        fn from(var: MaxTimeDelayCall) -> Self {
            b_contractCalls::MaxTimeDelay(var)
        }
    }
    impl ::std::convert::From<MinBlockDelayKeeperCall> for b_contractCalls {
        fn from(var: MinBlockDelayKeeperCall) -> Self {
            b_contractCalls::MinBlockDelayKeeper(var)
        }
    }
    impl ::std::convert::From<MinExecutionFeeCall> for b_contractCalls {
        fn from(var: MinExecutionFeeCall) -> Self {
            b_contractCalls::MinExecutionFee(var)
        }
    }
    impl ::std::convert::From<MinTimeDelayPublicCall> for b_contractCalls {
        fn from(var: MinTimeDelayPublicCall) -> Self {
            b_contractCalls::MinTimeDelayPublic(var)
        }
    }
    impl ::std::convert::From<ReferralStorageCall> for b_contractCalls {
        fn from(var: ReferralStorageCall) -> Self {
            b_contractCalls::ReferralStorage(var)
        }
    }
    impl ::std::convert::From<RouterCall> for b_contractCalls {
        fn from(var: RouterCall) -> Self {
            b_contractCalls::Router(var)
        }
    }
    impl ::std::convert::From<SendValueCall> for b_contractCalls {
        fn from(var: SendValueCall) -> Self {
            b_contractCalls::SendValue(var)
        }
    }
    impl ::std::convert::From<SetAdminCall> for b_contractCalls {
        fn from(var: SetAdminCall) -> Self {
            b_contractCalls::SetAdmin(var)
        }
    }
    impl ::std::convert::From<SetDelayValuesCall> for b_contractCalls {
        fn from(var: SetDelayValuesCall) -> Self {
            b_contractCalls::SetDelayValues(var)
        }
    }
    impl ::std::convert::From<SetDepositFeeCall> for b_contractCalls {
        fn from(var: SetDepositFeeCall) -> Self {
            b_contractCalls::SetDepositFee(var)
        }
    }
    impl ::std::convert::From<SetGovCall> for b_contractCalls {
        fn from(var: SetGovCall) -> Self {
            b_contractCalls::SetGov(var)
        }
    }
    impl ::std::convert::From<SetIncreasePositionBufferBpsCall> for b_contractCalls {
        fn from(var: SetIncreasePositionBufferBpsCall) -> Self {
            b_contractCalls::SetIncreasePositionBufferBps(var)
        }
    }
    impl ::std::convert::From<SetIsLeverageEnabledCall> for b_contractCalls {
        fn from(var: SetIsLeverageEnabledCall) -> Self {
            b_contractCalls::SetIsLeverageEnabled(var)
        }
    }
    impl ::std::convert::From<SetMaxGlobalSizesCall> for b_contractCalls {
        fn from(var: SetMaxGlobalSizesCall) -> Self {
            b_contractCalls::SetMaxGlobalSizes(var)
        }
    }
    impl ::std::convert::From<SetMinExecutionFeeCall> for b_contractCalls {
        fn from(var: SetMinExecutionFeeCall) -> Self {
            b_contractCalls::SetMinExecutionFee(var)
        }
    }
    impl ::std::convert::From<SetPositionKeeperCall> for b_contractCalls {
        fn from(var: SetPositionKeeperCall) -> Self {
            b_contractCalls::SetPositionKeeper(var)
        }
    }
    impl ::std::convert::From<SetReferralStorageCall> for b_contractCalls {
        fn from(var: SetReferralStorageCall) -> Self {
            b_contractCalls::SetReferralStorage(var)
        }
    }
    impl ::std::convert::From<SetRequestKeysStartValuesCall> for b_contractCalls {
        fn from(var: SetRequestKeysStartValuesCall) -> Self {
            b_contractCalls::SetRequestKeysStartValues(var)
        }
    }
    impl ::std::convert::From<VaultCall> for b_contractCalls {
        fn from(var: VaultCall) -> Self {
            b_contractCalls::Vault(var)
        }
    }
    impl ::std::convert::From<WethCall> for b_contractCalls {
        fn from(var: WethCall) -> Self {
            b_contractCalls::Weth(var)
        }
    }
    impl ::std::convert::From<WithdrawFeesCall> for b_contractCalls {
        fn from(var: WithdrawFeesCall) -> Self {
            b_contractCalls::WithdrawFees(var)
        }
    }
    #[doc = "Container type for all return fields from the `BASIS_POINTS_DIVISOR` function with signature `BASIS_POINTS_DIVISOR()` and selector `[18, 96, 130, 207]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct BasisPointsDivisorReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `admin` function with signature `admin()` and selector `[248, 81, 164, 64]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct AdminReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `cancelDecreasePosition` function with signature `cancelDecreasePosition(bytes32,address)` and selector `[96, 163, 98, 226]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct CancelDecreasePositionReturn(pub bool);
    #[doc = "Container type for all return fields from the `cancelIncreasePosition` function with signature `cancelIncreasePosition(bytes32,address)` and selector `[34, 95, 201, 253]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct CancelIncreasePositionReturn(pub bool);
    #[doc = "Container type for all return fields from the `decreasePositionRequestKeys` function with signature `decreasePositionRequestKeys(uint256)` and selector `[66, 120, 85, 95]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct DecreasePositionRequestKeysReturn(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `decreasePositionRequestKeysStart` function with signature `decreasePositionRequestKeysStart()` and selector `[27, 202, 140, 240]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct DecreasePositionRequestKeysStartReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `decreasePositionRequests` function with signature `decreasePositionRequests(bytes32)` and selector `[31, 40, 81, 6]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct DecreasePositionRequestsReturn {
        pub account: ethers::core::types::Address,
        pub index_token: ethers::core::types::Address,
        pub collateral_delta: ethers::core::types::U256,
        pub size_delta: ethers::core::types::U256,
        pub is_long: bool,
        pub receiver: ethers::core::types::Address,
        pub acceptable_price: ethers::core::types::U256,
        pub min_out: ethers::core::types::U256,
        pub execution_fee: ethers::core::types::U256,
        pub block_number: ethers::core::types::U256,
        pub block_time: ethers::core::types::U256,
        pub withdraw_eth: bool,
    }
    #[doc = "Container type for all return fields from the `decreasePositionsIndex` function with signature `decreasePositionsIndex(address)` and selector `[250, 68, 69, 119]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct DecreasePositionsIndexReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `depositFee` function with signature `depositFee()` and selector `[103, 165, 39, 147]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct DepositFeeReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `executeDecreasePosition` function with signature `executeDecreasePosition(bytes32,address)` and selector `[13, 77, 0, 61]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct ExecuteDecreasePositionReturn(pub bool);
    #[doc = "Container type for all return fields from the `executeIncreasePosition` function with signature `executeIncreasePosition(bytes32,address)` and selector `[39, 180, 44, 15]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct ExecuteIncreasePositionReturn(pub bool);
    #[doc = "Container type for all return fields from the `feeReserves` function with signature `feeReserves(address)` and selector `[28, 233, 203, 143]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct FeeReservesReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `getDecreasePositionRequestPath` function with signature `getDecreasePositionRequestPath(bytes32)` and selector `[93, 92, 34, 232]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetDecreasePositionRequestPathReturn(
        pub ::std::vec::Vec<ethers::core::types::Address>,
    );
    #[doc = "Container type for all return fields from the `getIncreasePositionRequestPath` function with signature `getIncreasePositionRequestPath(bytes32)` and selector `[149, 233, 187, 215]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetIncreasePositionRequestPathReturn(
        pub ::std::vec::Vec<ethers::core::types::Address>,
    );
    #[doc = "Container type for all return fields from the `getRequestKey` function with signature `getRequestKey(address,uint256)` and selector `[98, 248, 163, 254]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetRequestKeyReturn(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `getRequestQueueLengths` function with signature `getRequestQueueLengths()` and selector `[242, 206, 166, 165]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetRequestQueueLengthsReturn(
        pub ethers::core::types::U256,
        pub ethers::core::types::U256,
        pub ethers::core::types::U256,
        pub ethers::core::types::U256,
    );
    #[doc = "Container type for all return fields from the `gov` function with signature `gov()` and selector `[18, 212, 58, 81]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GovReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `increasePositionBufferBps` function with signature `increasePositionBufferBps()` and selector `[152, 209, 224, 58]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct IncreasePositionBufferBpsReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `increasePositionRequestKeys` function with signature `increasePositionRequestKeys(uint256)` and selector `[4, 34, 89, 84]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct IncreasePositionRequestKeysReturn(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `increasePositionRequestKeysStart` function with signature `increasePositionRequestKeysStart()` and selector `[155, 87, 134, 32]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct IncreasePositionRequestKeysStartReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `increasePositionRequests` function with signature `increasePositionRequests(bytes32)` and selector `[250, 249, 144, 243]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct IncreasePositionRequestsReturn {
        pub account: ethers::core::types::Address,
        pub index_token: ethers::core::types::Address,
        pub amount_in: ethers::core::types::U256,
        pub min_out: ethers::core::types::U256,
        pub size_delta: ethers::core::types::U256,
        pub is_long: bool,
        pub acceptable_price: ethers::core::types::U256,
        pub execution_fee: ethers::core::types::U256,
        pub block_number: ethers::core::types::U256,
        pub block_time: ethers::core::types::U256,
        pub has_collateral_in_eth: bool,
    }
    #[doc = "Container type for all return fields from the `increasePositionsIndex` function with signature `increasePositionsIndex(address)` and selector `[99, 52, 81, 222]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct IncreasePositionsIndexReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `isLeverageEnabled` function with signature `isLeverageEnabled()` and selector `[62, 114, 162, 98]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct IsLeverageEnabledReturn(pub bool);
    #[doc = "Container type for all return fields from the `isPositionKeeper` function with signature `isPositionKeeper(address)` and selector `[54, 235, 164, 138]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct IsPositionKeeperReturn(pub bool);
    #[doc = "Container type for all return fields from the `maxGlobalLongSizes` function with signature `maxGlobalLongSizes(address)` and selector `[16, 69, 199, 78]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct MaxGlobalLongSizesReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `maxGlobalShortSizes` function with signature `maxGlobalShortSizes(address)` and selector `[150, 152, 210, 90]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct MaxGlobalShortSizesReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `maxTimeDelay` function with signature `maxTimeDelay()` and selector `[203, 2, 105, 201]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct MaxTimeDelayReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `minBlockDelayKeeper` function with signature `minBlockDelayKeeper()` and selector `[88, 65, 252, 170]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct MinBlockDelayKeeperReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `minExecutionFee` function with signature `minExecutionFee()` and selector `[99, 174, 33, 3]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct MinExecutionFeeReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `minTimeDelayPublic` function with signature `minTimeDelayPublic()` and selector `[58, 42, 128, 199]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct MinTimeDelayPublicReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `referralStorage` function with signature `referralStorage()` and selector `[0, 108, 195, 94]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct ReferralStorageReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `router` function with signature `router()` and selector `[248, 135, 234, 64]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct RouterReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `vault` function with signature `vault()` and selector `[251, 250, 119, 207]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct VaultReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `weth` function with signature `weth()` and selector `[63, 200, 206, 243]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct WethReturn(pub ethers::core::types::Address);
}
