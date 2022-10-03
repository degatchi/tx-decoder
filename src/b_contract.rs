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
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"orderIndex\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"address\",\"name\":\"collateralToken\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"collateralDelta\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"address\",\"name\":\"indexToken\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"sizeDelta\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"bool\",\"name\":\"isLong\",\"type\":\"bool\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"triggerPrice\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"bool\",\"name\":\"triggerAboveThreshold\",\"type\":\"bool\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"executionFee\",\"type\":\"uint256\"}],\"name\":\"CancelDecreaseOrder\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"orderIndex\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"address\",\"name\":\"purchaseToken\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"purchaseTokenAmount\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"address\",\"name\":\"collateralToken\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"address\",\"name\":\"indexToken\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"sizeDelta\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"bool\",\"name\":\"isLong\",\"type\":\"bool\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"triggerPrice\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"bool\",\"name\":\"triggerAboveThreshold\",\"type\":\"bool\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"executionFee\",\"type\":\"uint256\"}],\"name\":\"CancelIncreaseOrder\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"orderIndex\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"address[]\",\"name\":\"path\",\"type\":\"address[]\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"amountIn\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"minOut\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"triggerRatio\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"bool\",\"name\":\"triggerAboveThreshold\",\"type\":\"bool\"},{\"indexed\":false,\"internalType\":\"bool\",\"name\":\"shouldUnwrap\",\"type\":\"bool\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"executionFee\",\"type\":\"uint256\"}],\"name\":\"CancelSwapOrder\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"orderIndex\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"address\",\"name\":\"collateralToken\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"collateralDelta\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"address\",\"name\":\"indexToken\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"sizeDelta\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"bool\",\"name\":\"isLong\",\"type\":\"bool\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"triggerPrice\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"bool\",\"name\":\"triggerAboveThreshold\",\"type\":\"bool\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"executionFee\",\"type\":\"uint256\"}],\"name\":\"CreateDecreaseOrder\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"orderIndex\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"address\",\"name\":\"purchaseToken\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"purchaseTokenAmount\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"address\",\"name\":\"collateralToken\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"address\",\"name\":\"indexToken\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"sizeDelta\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"bool\",\"name\":\"isLong\",\"type\":\"bool\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"triggerPrice\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"bool\",\"name\":\"triggerAboveThreshold\",\"type\":\"bool\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"executionFee\",\"type\":\"uint256\"}],\"name\":\"CreateIncreaseOrder\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"orderIndex\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"address[]\",\"name\":\"path\",\"type\":\"address[]\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"amountIn\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"minOut\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"triggerRatio\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"bool\",\"name\":\"triggerAboveThreshold\",\"type\":\"bool\"},{\"indexed\":false,\"internalType\":\"bool\",\"name\":\"shouldUnwrap\",\"type\":\"bool\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"executionFee\",\"type\":\"uint256\"}],\"name\":\"CreateSwapOrder\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"orderIndex\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"address\",\"name\":\"collateralToken\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"collateralDelta\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"address\",\"name\":\"indexToken\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"sizeDelta\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"bool\",\"name\":\"isLong\",\"type\":\"bool\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"triggerPrice\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"bool\",\"name\":\"triggerAboveThreshold\",\"type\":\"bool\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"executionFee\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"executionPrice\",\"type\":\"uint256\"}],\"name\":\"ExecuteDecreaseOrder\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"orderIndex\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"address\",\"name\":\"purchaseToken\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"purchaseTokenAmount\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"address\",\"name\":\"collateralToken\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"address\",\"name\":\"indexToken\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"sizeDelta\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"bool\",\"name\":\"isLong\",\"type\":\"bool\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"triggerPrice\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"bool\",\"name\":\"triggerAboveThreshold\",\"type\":\"bool\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"executionFee\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"executionPrice\",\"type\":\"uint256\"}],\"name\":\"ExecuteIncreaseOrder\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"orderIndex\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"address[]\",\"name\":\"path\",\"type\":\"address[]\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"amountIn\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"minOut\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"amountOut\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"triggerRatio\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"bool\",\"name\":\"triggerAboveThreshold\",\"type\":\"bool\"},{\"indexed\":false,\"internalType\":\"bool\",\"name\":\"shouldUnwrap\",\"type\":\"bool\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"executionFee\",\"type\":\"uint256\"}],\"name\":\"ExecuteSwapOrder\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"address\",\"name\":\"router\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"address\",\"name\":\"vault\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"address\",\"name\":\"weth\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"address\",\"name\":\"usdg\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"minExecutionFee\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"minPurchaseTokenAmountUsd\",\"type\":\"uint256\"}],\"name\":\"Initialize\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"orderIndex\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"address\",\"name\":\"collateralToken\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"collateralDelta\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"address\",\"name\":\"indexToken\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"sizeDelta\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"bool\",\"name\":\"isLong\",\"type\":\"bool\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"triggerPrice\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"bool\",\"name\":\"triggerAboveThreshold\",\"type\":\"bool\"}],\"name\":\"UpdateDecreaseOrder\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"address\",\"name\":\"gov\",\"type\":\"address\"}],\"name\":\"UpdateGov\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"orderIndex\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"address\",\"name\":\"collateralToken\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"address\",\"name\":\"indexToken\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"bool\",\"name\":\"isLong\",\"type\":\"bool\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"sizeDelta\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"triggerPrice\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"bool\",\"name\":\"triggerAboveThreshold\",\"type\":\"bool\"}],\"name\":\"UpdateIncreaseOrder\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"minExecutionFee\",\"type\":\"uint256\"}],\"name\":\"UpdateMinExecutionFee\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"minPurchaseTokenAmountUsd\",\"type\":\"uint256\"}],\"name\":\"UpdateMinPurchaseTokenAmountUsd\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"ordexIndex\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"address[]\",\"name\":\"path\",\"type\":\"address[]\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"amountIn\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"minOut\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"triggerRatio\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"bool\",\"name\":\"triggerAboveThreshold\",\"type\":\"bool\"},{\"indexed\":false,\"internalType\":\"bool\",\"name\":\"shouldUnwrap\",\"type\":\"bool\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"executionFee\",\"type\":\"uint256\"}],\"name\":\"UpdateSwapOrder\",\"type\":\"event\"},{\"inputs\":[],\"name\":\"PRICE_PRECISION\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"USDG_PRECISION\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_orderIndex\",\"type\":\"uint256\"}],\"name\":\"cancelDecreaseOrder\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_orderIndex\",\"type\":\"uint256\"}],\"name\":\"cancelIncreaseOrder\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256[]\",\"name\":\"_swapOrderIndexes\",\"type\":\"uint256[]\"},{\"internalType\":\"uint256[]\",\"name\":\"_increaseOrderIndexes\",\"type\":\"uint256[]\"},{\"internalType\":\"uint256[]\",\"name\":\"_decreaseOrderIndexes\",\"type\":\"uint256[]\"}],\"name\":\"cancelMultiple\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_orderIndex\",\"type\":\"uint256\"}],\"name\":\"cancelSwapOrder\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_indexToken\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"_sizeDelta\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"_collateralToken\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"_collateralDelta\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"_isLong\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"_triggerPrice\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"_triggerAboveThreshold\",\"type\":\"bool\"}],\"name\":\"createDecreaseOrder\",\"outputs\":[],\"stateMutability\":\"payable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address[]\",\"name\":\"_path\",\"type\":\"address[]\"},{\"internalType\":\"uint256\",\"name\":\"_amountIn\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"_indexToken\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"_minOut\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"_sizeDelta\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"_collateralToken\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"_isLong\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"_triggerPrice\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"_triggerAboveThreshold\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"_executionFee\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"_shouldWrap\",\"type\":\"bool\"}],\"name\":\"createIncreaseOrder\",\"outputs\":[],\"stateMutability\":\"payable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address[]\",\"name\":\"_path\",\"type\":\"address[]\"},{\"internalType\":\"uint256\",\"name\":\"_amountIn\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"_minOut\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"_triggerRatio\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"_triggerAboveThreshold\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"_executionFee\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"_shouldWrap\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"_shouldUnwrap\",\"type\":\"bool\"}],\"name\":\"createSwapOrder\",\"outputs\":[],\"stateMutability\":\"payable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"name\":\"decreaseOrders\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"collateralToken\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"collateralDelta\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"indexToken\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"sizeDelta\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"isLong\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"triggerPrice\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"triggerAboveThreshold\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"executionFee\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"name\":\"decreaseOrdersIndex\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_address\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"_orderIndex\",\"type\":\"uint256\"},{\"internalType\":\"address payable\",\"name\":\"_feeReceiver\",\"type\":\"address\"}],\"name\":\"executeDecreaseOrder\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_address\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"_orderIndex\",\"type\":\"uint256\"},{\"internalType\":\"address payable\",\"name\":\"_feeReceiver\",\"type\":\"address\"}],\"name\":\"executeIncreaseOrder\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_account\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"_orderIndex\",\"type\":\"uint256\"},{\"internalType\":\"address payable\",\"name\":\"_feeReceiver\",\"type\":\"address\"}],\"name\":\"executeSwapOrder\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_account\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"_orderIndex\",\"type\":\"uint256\"}],\"name\":\"getDecreaseOrder\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"collateralToken\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"collateralDelta\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"indexToken\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"sizeDelta\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"isLong\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"triggerPrice\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"triggerAboveThreshold\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"executionFee\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_account\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"_orderIndex\",\"type\":\"uint256\"}],\"name\":\"getIncreaseOrder\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"purchaseToken\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"purchaseTokenAmount\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"collateralToken\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"indexToken\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"sizeDelta\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"isLong\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"triggerPrice\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"triggerAboveThreshold\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"executionFee\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_account\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"_orderIndex\",\"type\":\"uint256\"}],\"name\":\"getSwapOrder\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"path0\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"path1\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"path2\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"amountIn\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"minOut\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"triggerRatio\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"triggerAboveThreshold\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"shouldUnwrap\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"executionFee\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_otherToken\",\"type\":\"address\"}],\"name\":\"getUsdgMinPrice\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"gov\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"name\":\"increaseOrders\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"purchaseToken\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"purchaseTokenAmount\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"collateralToken\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"indexToken\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"sizeDelta\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"isLong\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"triggerPrice\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"triggerAboveThreshold\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"executionFee\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"name\":\"increaseOrdersIndex\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_router\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"_vault\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"_weth\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"_usdg\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"_minExecutionFee\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"_minPurchaseTokenAmountUsd\",\"type\":\"uint256\"}],\"name\":\"initialize\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"isInitialized\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"minExecutionFee\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"minPurchaseTokenAmountUsd\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"router\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_gov\",\"type\":\"address\"}],\"name\":\"setGov\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_minExecutionFee\",\"type\":\"uint256\"}],\"name\":\"setMinExecutionFee\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_minPurchaseTokenAmountUsd\",\"type\":\"uint256\"}],\"name\":\"setMinPurchaseTokenAmountUsd\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"name\":\"swapOrders\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"amountIn\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"minOut\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"triggerRatio\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"triggerAboveThreshold\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"shouldUnwrap\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"executionFee\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"name\":\"swapOrdersIndex\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_orderIndex\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"_collateralDelta\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"_sizeDelta\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"_triggerPrice\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"_triggerAboveThreshold\",\"type\":\"bool\"}],\"name\":\"updateDecreaseOrder\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_orderIndex\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"_sizeDelta\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"_triggerPrice\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"_triggerAboveThreshold\",\"type\":\"bool\"}],\"name\":\"updateIncreaseOrder\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_orderIndex\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"_minOut\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"_triggerRatio\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"_triggerAboveThreshold\",\"type\":\"bool\"}],\"name\":\"updateSwapOrder\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"usdg\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"_triggerAboveThreshold\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"_triggerPrice\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"_indexToken\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"_maximizePrice\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"_raise\",\"type\":\"bool\"}],\"name\":\"validatePositionOrderPrice\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address[]\",\"name\":\"_path\",\"type\":\"address[]\"},{\"internalType\":\"uint256\",\"name\":\"_triggerRatio\",\"type\":\"uint256\"}],\"name\":\"validateSwapOrderPriceWithTriggerAboveThreshold\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"vault\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"weth\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"stateMutability\":\"payable\",\"type\":\"receive\"}]") . expect ("invalid abi")
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
        #[doc = "Calls the contract's `PRICE_PRECISION` (0x95082d25) function"]
        pub fn price_precision(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([149, 8, 45, 37], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `USDG_PRECISION` (0x4a686d67) function"]
        pub fn usdg_precision(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([74, 104, 109, 103], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `cancelDecreaseOrder` (0x9e71b0f0) function"]
        pub fn cancel_decrease_order(
            &self,
            order_index: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([158, 113, 176, 240], order_index)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `cancelIncreaseOrder` (0x47e0bbd0) function"]
        pub fn cancel_increase_order(
            &self,
            order_index: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([71, 224, 187, 208], order_index)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `cancelMultiple` (0x807c5600) function"]
        pub fn cancel_multiple(
            &self,
            swap_order_indexes: ::std::vec::Vec<ethers::core::types::U256>,
            increase_order_indexes: ::std::vec::Vec<ethers::core::types::U256>,
            decrease_order_indexes: ::std::vec::Vec<ethers::core::types::U256>,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [128, 124, 86, 0],
                    (
                        swap_order_indexes,
                        increase_order_indexes,
                        decrease_order_indexes,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `cancelSwapOrder` (0xf882ac07) function"]
        pub fn cancel_swap_order(
            &self,
            order_index: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([248, 130, 172, 7], order_index)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `createDecreaseOrder` (0xc16cde8a) function"]
        pub fn create_decrease_order(
            &self,
            index_token: ethers::core::types::Address,
            size_delta: ethers::core::types::U256,
            collateral_token: ethers::core::types::Address,
            collateral_delta: ethers::core::types::U256,
            is_long: bool,
            trigger_price: ethers::core::types::U256,
            trigger_above_threshold: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [193, 108, 222, 138],
                    (
                        index_token,
                        size_delta,
                        collateral_token,
                        collateral_delta,
                        is_long,
                        trigger_price,
                        trigger_above_threshold,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `createIncreaseOrder` (0xb142a4b0) function"]
        pub fn create_increase_order(
            &self,
            path: ::std::vec::Vec<ethers::core::types::Address>,
            amount_in: ethers::core::types::U256,
            index_token: ethers::core::types::Address,
            min_out: ethers::core::types::U256,
            size_delta: ethers::core::types::U256,
            collateral_token: ethers::core::types::Address,
            is_long: bool,
            trigger_price: ethers::core::types::U256,
            trigger_above_threshold: bool,
            execution_fee: ethers::core::types::U256,
            should_wrap: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [177, 66, 164, 176],
                    (
                        path,
                        amount_in,
                        index_token,
                        min_out,
                        size_delta,
                        collateral_token,
                        is_long,
                        trigger_price,
                        trigger_above_threshold,
                        execution_fee,
                        should_wrap,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `createSwapOrder` (0x269ae6c2) function"]
        pub fn create_swap_order(
            &self,
            path: ::std::vec::Vec<ethers::core::types::Address>,
            amount_in: ethers::core::types::U256,
            min_out: ethers::core::types::U256,
            trigger_ratio: ethers::core::types::U256,
            trigger_above_threshold: bool,
            execution_fee: ethers::core::types::U256,
            should_wrap: bool,
            should_unwrap: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [38, 154, 230, 194],
                    (
                        path,
                        amount_in,
                        min_out,
                        trigger_ratio,
                        trigger_above_threshold,
                        execution_fee,
                        should_wrap,
                        should_unwrap,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `decreaseOrders` (0xf2d2e01b) function"]
        pub fn decrease_orders(
            &self,
            p0: ethers::core::types::Address,
            p1: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                ethers::core::types::Address,
                ethers::core::types::Address,
                ethers::core::types::U256,
                ethers::core::types::Address,
                ethers::core::types::U256,
                bool,
                ethers::core::types::U256,
                bool,
                ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([242, 210, 224, 27], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `decreaseOrdersIndex` (0xd566d0ca) function"]
        pub fn decrease_orders_index(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([213, 102, 208, 202], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `executeDecreaseOrder` (0x11d9444a) function"]
        pub fn execute_decrease_order(
            &self,
            address: ethers::core::types::Address,
            order_index: ethers::core::types::U256,
            fee_receiver: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([17, 217, 68, 74], (address, order_index, fee_receiver))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `executeIncreaseOrder` (0xd38ab519) function"]
        pub fn execute_increase_order(
            &self,
            address: ethers::core::types::Address,
            order_index: ethers::core::types::U256,
            fee_receiver: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([211, 138, 181, 25], (address, order_index, fee_receiver))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `executeSwapOrder` (0x07c7edc3) function"]
        pub fn execute_swap_order(
            &self,
            account: ethers::core::types::Address,
            order_index: ethers::core::types::U256,
            fee_receiver: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([7, 199, 237, 195], (account, order_index, fee_receiver))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getDecreaseOrder` (0x026032ee) function"]
        pub fn get_decrease_order(
            &self,
            account: ethers::core::types::Address,
            order_index: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                ethers::core::types::Address,
                ethers::core::types::U256,
                ethers::core::types::Address,
                ethers::core::types::U256,
                bool,
                ethers::core::types::U256,
                bool,
                ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([2, 96, 50, 238], (account, order_index))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getIncreaseOrder` (0xd3bab1d1) function"]
        pub fn get_increase_order(
            &self,
            account: ethers::core::types::Address,
            order_index: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                ethers::core::types::Address,
                ethers::core::types::U256,
                ethers::core::types::Address,
                ethers::core::types::Address,
                ethers::core::types::U256,
                bool,
                ethers::core::types::U256,
                bool,
                ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([211, 186, 177, 209], (account, order_index))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getSwapOrder` (0xd0d40cd6) function"]
        pub fn get_swap_order(
            &self,
            account: ethers::core::types::Address,
            order_index: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                ethers::core::types::Address,
                ethers::core::types::Address,
                ethers::core::types::Address,
                ethers::core::types::U256,
                ethers::core::types::U256,
                ethers::core::types::U256,
                bool,
                bool,
                ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([208, 212, 12, 214], (account, order_index))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getUsdgMinPrice` (0x9e23de5c) function"]
        pub fn get_usdg_min_price(
            &self,
            other_token: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([158, 35, 222, 92], other_token)
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
        #[doc = "Calls the contract's `increaseOrders` (0x2b7d6290) function"]
        pub fn increase_orders(
            &self,
            p0: ethers::core::types::Address,
            p1: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                ethers::core::types::Address,
                ethers::core::types::Address,
                ethers::core::types::U256,
                ethers::core::types::Address,
                ethers::core::types::Address,
                ethers::core::types::U256,
                bool,
                ethers::core::types::U256,
                bool,
                ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([43, 125, 98, 144], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `increaseOrdersIndex` (0xaec22455) function"]
        pub fn increase_orders_index(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([174, 194, 36, 85], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `initialize` (0xd7c41c79) function"]
        pub fn initialize(
            &self,
            router: ethers::core::types::Address,
            vault: ethers::core::types::Address,
            weth: ethers::core::types::Address,
            usdg: ethers::core::types::Address,
            min_execution_fee: ethers::core::types::U256,
            min_purchase_token_amount_usd: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [215, 196, 28, 121],
                    (
                        router,
                        vault,
                        weth,
                        usdg,
                        min_execution_fee,
                        min_purchase_token_amount_usd,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isInitialized` (0x392e53cd) function"]
        pub fn is_initialized(&self) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([57, 46, 83, 205], ())
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
        #[doc = "Calls the contract's `minPurchaseTokenAmountUsd` (0x8de10c2e) function"]
        pub fn min_purchase_token_amount_usd(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([141, 225, 12, 46], ())
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
        #[doc = "Calls the contract's `setGov` (0xcfad57a2) function"]
        pub fn set_gov(
            &self,
            gov: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([207, 173, 87, 162], gov)
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
        #[doc = "Calls the contract's `setMinPurchaseTokenAmountUsd` (0x0d5cc938) function"]
        pub fn set_min_purchase_token_amount_usd(
            &self,
            min_purchase_token_amount_usd: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([13, 92, 201, 56], min_purchase_token_amount_usd)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `swapOrders` (0x79221fa2) function"]
        pub fn swap_orders(
            &self,
            p0: ethers::core::types::Address,
            p1: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                ethers::core::types::Address,
                ethers::core::types::U256,
                ethers::core::types::U256,
                ethers::core::types::U256,
                bool,
                bool,
                ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([121, 34, 31, 162], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `swapOrdersIndex` (0x00cf066b) function"]
        pub fn swap_orders_index(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([0, 207, 6, 107], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `updateDecreaseOrder` (0xa397ea54) function"]
        pub fn update_decrease_order(
            &self,
            order_index: ethers::core::types::U256,
            collateral_delta: ethers::core::types::U256,
            size_delta: ethers::core::types::U256,
            trigger_price: ethers::core::types::U256,
            trigger_above_threshold: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [163, 151, 234, 84],
                    (
                        order_index,
                        collateral_delta,
                        size_delta,
                        trigger_price,
                        trigger_above_threshold,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `updateIncreaseOrder` (0x9983ee1b) function"]
        pub fn update_increase_order(
            &self,
            order_index: ethers::core::types::U256,
            size_delta: ethers::core::types::U256,
            trigger_price: ethers::core::types::U256,
            trigger_above_threshold: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [153, 131, 238, 27],
                    (
                        order_index,
                        size_delta,
                        trigger_price,
                        trigger_above_threshold,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `updateSwapOrder` (0xc86b0f7d) function"]
        pub fn update_swap_order(
            &self,
            order_index: ethers::core::types::U256,
            min_out: ethers::core::types::U256,
            trigger_ratio: ethers::core::types::U256,
            trigger_above_threshold: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [200, 107, 15, 125],
                    (order_index, min_out, trigger_ratio, trigger_above_threshold),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `usdg` (0xf5b91b7b) function"]
        pub fn usdg(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([245, 185, 27, 123], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `validatePositionOrderPrice` (0x4c54f0b0) function"]
        pub fn validate_position_order_price(
            &self,
            trigger_above_threshold: bool,
            trigger_price: ethers::core::types::U256,
            index_token: ethers::core::types::Address,
            maximize_price: bool,
            raise: bool,
        ) -> ethers::contract::builders::ContractCall<M, (ethers::core::types::U256, bool)>
        {
            self.0
                .method_hash(
                    [76, 84, 240, 176],
                    (
                        trigger_above_threshold,
                        trigger_price,
                        index_token,
                        maximize_price,
                        raise,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `validateSwapOrderPriceWithTriggerAboveThreshold` (0xc4a1821b) function"]
        pub fn validate_swap_order_price_with_trigger_above_threshold(
            &self,
            path: ::std::vec::Vec<ethers::core::types::Address>,
            trigger_ratio: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([196, 161, 130, 27], (path, trigger_ratio))
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
        #[doc = "Gets the contract's `CancelDecreaseOrder` event"]
        pub fn cancel_decrease_order_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, CancelDecreaseOrderFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `CancelIncreaseOrder` event"]
        pub fn cancel_increase_order_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, CancelIncreaseOrderFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `CancelSwapOrder` event"]
        pub fn cancel_swap_order_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, CancelSwapOrderFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `CreateDecreaseOrder` event"]
        pub fn create_decrease_order_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, CreateDecreaseOrderFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `CreateIncreaseOrder` event"]
        pub fn create_increase_order_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, CreateIncreaseOrderFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `CreateSwapOrder` event"]
        pub fn create_swap_order_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, CreateSwapOrderFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ExecuteDecreaseOrder` event"]
        pub fn execute_decrease_order_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ExecuteDecreaseOrderFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ExecuteIncreaseOrder` event"]
        pub fn execute_increase_order_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ExecuteIncreaseOrderFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ExecuteSwapOrder` event"]
        pub fn execute_swap_order_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ExecuteSwapOrderFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Initialize` event"]
        pub fn initialize_filter(&self) -> ethers::contract::builders::Event<M, InitializeFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `UpdateDecreaseOrder` event"]
        pub fn update_decrease_order_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, UpdateDecreaseOrderFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `UpdateGov` event"]
        pub fn update_gov_filter(&self) -> ethers::contract::builders::Event<M, UpdateGovFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `UpdateIncreaseOrder` event"]
        pub fn update_increase_order_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, UpdateIncreaseOrderFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `UpdateMinExecutionFee` event"]
        pub fn update_min_execution_fee_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, UpdateMinExecutionFeeFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `UpdateMinPurchaseTokenAmountUsd` event"]
        pub fn update_min_purchase_token_amount_usd_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, UpdateMinPurchaseTokenAmountUsdFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `UpdateSwapOrder` event"]
        pub fn update_swap_order_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, UpdateSwapOrderFilter> {
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
        name = "CancelDecreaseOrder",
        abi = "CancelDecreaseOrder(address,uint256,address,uint256,address,uint256,bool,uint256,bool,uint256)"
    )]
    pub struct CancelDecreaseOrderFilter {
        #[ethevent(indexed)]
        pub account: ethers::core::types::Address,
        pub order_index: ethers::core::types::U256,
        pub collateral_token: ethers::core::types::Address,
        pub collateral_delta: ethers::core::types::U256,
        pub index_token: ethers::core::types::Address,
        pub size_delta: ethers::core::types::U256,
        pub is_long: bool,
        pub trigger_price: ethers::core::types::U256,
        pub trigger_above_threshold: bool,
        pub execution_fee: ethers::core::types::U256,
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
        name = "CancelIncreaseOrder",
        abi = "CancelIncreaseOrder(address,uint256,address,uint256,address,address,uint256,bool,uint256,bool,uint256)"
    )]
    pub struct CancelIncreaseOrderFilter {
        #[ethevent(indexed)]
        pub account: ethers::core::types::Address,
        pub order_index: ethers::core::types::U256,
        pub purchase_token: ethers::core::types::Address,
        pub purchase_token_amount: ethers::core::types::U256,
        pub collateral_token: ethers::core::types::Address,
        pub index_token: ethers::core::types::Address,
        pub size_delta: ethers::core::types::U256,
        pub is_long: bool,
        pub trigger_price: ethers::core::types::U256,
        pub trigger_above_threshold: bool,
        pub execution_fee: ethers::core::types::U256,
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
        name = "CancelSwapOrder",
        abi = "CancelSwapOrder(address,uint256,address[],uint256,uint256,uint256,bool,bool,uint256)"
    )]
    pub struct CancelSwapOrderFilter {
        #[ethevent(indexed)]
        pub account: ethers::core::types::Address,
        pub order_index: ethers::core::types::U256,
        pub path: Vec<ethers::core::types::Address>,
        pub amount_in: ethers::core::types::U256,
        pub min_out: ethers::core::types::U256,
        pub trigger_ratio: ethers::core::types::U256,
        pub trigger_above_threshold: bool,
        pub should_unwrap: bool,
        pub execution_fee: ethers::core::types::U256,
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
        name = "CreateDecreaseOrder",
        abi = "CreateDecreaseOrder(address,uint256,address,uint256,address,uint256,bool,uint256,bool,uint256)"
    )]
    pub struct CreateDecreaseOrderFilter {
        #[ethevent(indexed)]
        pub account: ethers::core::types::Address,
        pub order_index: ethers::core::types::U256,
        pub collateral_token: ethers::core::types::Address,
        pub collateral_delta: ethers::core::types::U256,
        pub index_token: ethers::core::types::Address,
        pub size_delta: ethers::core::types::U256,
        pub is_long: bool,
        pub trigger_price: ethers::core::types::U256,
        pub trigger_above_threshold: bool,
        pub execution_fee: ethers::core::types::U256,
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
        name = "CreateIncreaseOrder",
        abi = "CreateIncreaseOrder(address,uint256,address,uint256,address,address,uint256,bool,uint256,bool,uint256)"
    )]
    pub struct CreateIncreaseOrderFilter {
        #[ethevent(indexed)]
        pub account: ethers::core::types::Address,
        pub order_index: ethers::core::types::U256,
        pub purchase_token: ethers::core::types::Address,
        pub purchase_token_amount: ethers::core::types::U256,
        pub collateral_token: ethers::core::types::Address,
        pub index_token: ethers::core::types::Address,
        pub size_delta: ethers::core::types::U256,
        pub is_long: bool,
        pub trigger_price: ethers::core::types::U256,
        pub trigger_above_threshold: bool,
        pub execution_fee: ethers::core::types::U256,
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
        name = "CreateSwapOrder",
        abi = "CreateSwapOrder(address,uint256,address[],uint256,uint256,uint256,bool,bool,uint256)"
    )]
    pub struct CreateSwapOrderFilter {
        #[ethevent(indexed)]
        pub account: ethers::core::types::Address,
        pub order_index: ethers::core::types::U256,
        pub path: Vec<ethers::core::types::Address>,
        pub amount_in: ethers::core::types::U256,
        pub min_out: ethers::core::types::U256,
        pub trigger_ratio: ethers::core::types::U256,
        pub trigger_above_threshold: bool,
        pub should_unwrap: bool,
        pub execution_fee: ethers::core::types::U256,
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
        name = "ExecuteDecreaseOrder",
        abi = "ExecuteDecreaseOrder(address,uint256,address,uint256,address,uint256,bool,uint256,bool,uint256,uint256)"
    )]
    pub struct ExecuteDecreaseOrderFilter {
        #[ethevent(indexed)]
        pub account: ethers::core::types::Address,
        pub order_index: ethers::core::types::U256,
        pub collateral_token: ethers::core::types::Address,
        pub collateral_delta: ethers::core::types::U256,
        pub index_token: ethers::core::types::Address,
        pub size_delta: ethers::core::types::U256,
        pub is_long: bool,
        pub trigger_price: ethers::core::types::U256,
        pub trigger_above_threshold: bool,
        pub execution_fee: ethers::core::types::U256,
        pub execution_price: ethers::core::types::U256,
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
        name = "ExecuteIncreaseOrder",
        abi = "ExecuteIncreaseOrder(address,uint256,address,uint256,address,address,uint256,bool,uint256,bool,uint256,uint256)"
    )]
    pub struct ExecuteIncreaseOrderFilter {
        #[ethevent(indexed)]
        pub account: ethers::core::types::Address,
        pub order_index: ethers::core::types::U256,
        pub purchase_token: ethers::core::types::Address,
        pub purchase_token_amount: ethers::core::types::U256,
        pub collateral_token: ethers::core::types::Address,
        pub index_token: ethers::core::types::Address,
        pub size_delta: ethers::core::types::U256,
        pub is_long: bool,
        pub trigger_price: ethers::core::types::U256,
        pub trigger_above_threshold: bool,
        pub execution_fee: ethers::core::types::U256,
        pub execution_price: ethers::core::types::U256,
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
        name = "ExecuteSwapOrder",
        abi = "ExecuteSwapOrder(address,uint256,address[],uint256,uint256,uint256,uint256,bool,bool,uint256)"
    )]
    pub struct ExecuteSwapOrderFilter {
        #[ethevent(indexed)]
        pub account: ethers::core::types::Address,
        pub order_index: ethers::core::types::U256,
        pub path: Vec<ethers::core::types::Address>,
        pub amount_in: ethers::core::types::U256,
        pub min_out: ethers::core::types::U256,
        pub amount_out: ethers::core::types::U256,
        pub trigger_ratio: ethers::core::types::U256,
        pub trigger_above_threshold: bool,
        pub should_unwrap: bool,
        pub execution_fee: ethers::core::types::U256,
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
        name = "Initialize",
        abi = "Initialize(address,address,address,address,uint256,uint256)"
    )]
    pub struct InitializeFilter {
        pub router: ethers::core::types::Address,
        pub vault: ethers::core::types::Address,
        pub weth: ethers::core::types::Address,
        pub usdg: ethers::core::types::Address,
        pub min_execution_fee: ethers::core::types::U256,
        pub min_purchase_token_amount_usd: ethers::core::types::U256,
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
        name = "UpdateDecreaseOrder",
        abi = "UpdateDecreaseOrder(address,uint256,address,uint256,address,uint256,bool,uint256,bool)"
    )]
    pub struct UpdateDecreaseOrderFilter {
        #[ethevent(indexed)]
        pub account: ethers::core::types::Address,
        pub order_index: ethers::core::types::U256,
        pub collateral_token: ethers::core::types::Address,
        pub collateral_delta: ethers::core::types::U256,
        pub index_token: ethers::core::types::Address,
        pub size_delta: ethers::core::types::U256,
        pub is_long: bool,
        pub trigger_price: ethers::core::types::U256,
        pub trigger_above_threshold: bool,
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
    #[ethevent(name = "UpdateGov", abi = "UpdateGov(address)")]
    pub struct UpdateGovFilter {
        pub gov: ethers::core::types::Address,
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
        name = "UpdateIncreaseOrder",
        abi = "UpdateIncreaseOrder(address,uint256,address,address,bool,uint256,uint256,bool)"
    )]
    pub struct UpdateIncreaseOrderFilter {
        #[ethevent(indexed)]
        pub account: ethers::core::types::Address,
        pub order_index: ethers::core::types::U256,
        pub collateral_token: ethers::core::types::Address,
        pub index_token: ethers::core::types::Address,
        pub is_long: bool,
        pub size_delta: ethers::core::types::U256,
        pub trigger_price: ethers::core::types::U256,
        pub trigger_above_threshold: bool,
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
    #[ethevent(name = "UpdateMinExecutionFee", abi = "UpdateMinExecutionFee(uint256)")]
    pub struct UpdateMinExecutionFeeFilter {
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
    #[ethevent(
        name = "UpdateMinPurchaseTokenAmountUsd",
        abi = "UpdateMinPurchaseTokenAmountUsd(uint256)"
    )]
    pub struct UpdateMinPurchaseTokenAmountUsdFilter {
        pub min_purchase_token_amount_usd: ethers::core::types::U256,
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
        name = "UpdateSwapOrder",
        abi = "UpdateSwapOrder(address,uint256,address[],uint256,uint256,uint256,bool,bool,uint256)"
    )]
    pub struct UpdateSwapOrderFilter {
        #[ethevent(indexed)]
        pub account: ethers::core::types::Address,
        pub ordex_index: ethers::core::types::U256,
        pub path: Vec<ethers::core::types::Address>,
        pub amount_in: ethers::core::types::U256,
        pub min_out: ethers::core::types::U256,
        pub trigger_ratio: ethers::core::types::U256,
        pub trigger_above_threshold: bool,
        pub should_unwrap: bool,
        pub execution_fee: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum b_contractEvents {
        CancelDecreaseOrderFilter(CancelDecreaseOrderFilter),
        CancelIncreaseOrderFilter(CancelIncreaseOrderFilter),
        CancelSwapOrderFilter(CancelSwapOrderFilter),
        CreateDecreaseOrderFilter(CreateDecreaseOrderFilter),
        CreateIncreaseOrderFilter(CreateIncreaseOrderFilter),
        CreateSwapOrderFilter(CreateSwapOrderFilter),
        ExecuteDecreaseOrderFilter(ExecuteDecreaseOrderFilter),
        ExecuteIncreaseOrderFilter(ExecuteIncreaseOrderFilter),
        ExecuteSwapOrderFilter(ExecuteSwapOrderFilter),
        InitializeFilter(InitializeFilter),
        UpdateDecreaseOrderFilter(UpdateDecreaseOrderFilter),
        UpdateGovFilter(UpdateGovFilter),
        UpdateIncreaseOrderFilter(UpdateIncreaseOrderFilter),
        UpdateMinExecutionFeeFilter(UpdateMinExecutionFeeFilter),
        UpdateMinPurchaseTokenAmountUsdFilter(UpdateMinPurchaseTokenAmountUsdFilter),
        UpdateSwapOrderFilter(UpdateSwapOrderFilter),
    }
    impl ethers::contract::EthLogDecode for b_contractEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = CancelDecreaseOrderFilter::decode_log(log) {
                return Ok(b_contractEvents::CancelDecreaseOrderFilter(decoded));
            }
            if let Ok(decoded) = CancelIncreaseOrderFilter::decode_log(log) {
                return Ok(b_contractEvents::CancelIncreaseOrderFilter(decoded));
            }
            if let Ok(decoded) = CancelSwapOrderFilter::decode_log(log) {
                return Ok(b_contractEvents::CancelSwapOrderFilter(decoded));
            }
            if let Ok(decoded) = CreateDecreaseOrderFilter::decode_log(log) {
                return Ok(b_contractEvents::CreateDecreaseOrderFilter(decoded));
            }
            if let Ok(decoded) = CreateIncreaseOrderFilter::decode_log(log) {
                return Ok(b_contractEvents::CreateIncreaseOrderFilter(decoded));
            }
            if let Ok(decoded) = CreateSwapOrderFilter::decode_log(log) {
                return Ok(b_contractEvents::CreateSwapOrderFilter(decoded));
            }
            if let Ok(decoded) = ExecuteDecreaseOrderFilter::decode_log(log) {
                return Ok(b_contractEvents::ExecuteDecreaseOrderFilter(decoded));
            }
            if let Ok(decoded) = ExecuteIncreaseOrderFilter::decode_log(log) {
                return Ok(b_contractEvents::ExecuteIncreaseOrderFilter(decoded));
            }
            if let Ok(decoded) = ExecuteSwapOrderFilter::decode_log(log) {
                return Ok(b_contractEvents::ExecuteSwapOrderFilter(decoded));
            }
            if let Ok(decoded) = InitializeFilter::decode_log(log) {
                return Ok(b_contractEvents::InitializeFilter(decoded));
            }
            if let Ok(decoded) = UpdateDecreaseOrderFilter::decode_log(log) {
                return Ok(b_contractEvents::UpdateDecreaseOrderFilter(decoded));
            }
            if let Ok(decoded) = UpdateGovFilter::decode_log(log) {
                return Ok(b_contractEvents::UpdateGovFilter(decoded));
            }
            if let Ok(decoded) = UpdateIncreaseOrderFilter::decode_log(log) {
                return Ok(b_contractEvents::UpdateIncreaseOrderFilter(decoded));
            }
            if let Ok(decoded) = UpdateMinExecutionFeeFilter::decode_log(log) {
                return Ok(b_contractEvents::UpdateMinExecutionFeeFilter(decoded));
            }
            if let Ok(decoded) = UpdateMinPurchaseTokenAmountUsdFilter::decode_log(log) {
                return Ok(b_contractEvents::UpdateMinPurchaseTokenAmountUsdFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = UpdateSwapOrderFilter::decode_log(log) {
                return Ok(b_contractEvents::UpdateSwapOrderFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for b_contractEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                b_contractEvents::CancelDecreaseOrderFilter(element) => element.fmt(f),
                b_contractEvents::CancelIncreaseOrderFilter(element) => element.fmt(f),
                b_contractEvents::CancelSwapOrderFilter(element) => element.fmt(f),
                b_contractEvents::CreateDecreaseOrderFilter(element) => element.fmt(f),
                b_contractEvents::CreateIncreaseOrderFilter(element) => element.fmt(f),
                b_contractEvents::CreateSwapOrderFilter(element) => element.fmt(f),
                b_contractEvents::ExecuteDecreaseOrderFilter(element) => element.fmt(f),
                b_contractEvents::ExecuteIncreaseOrderFilter(element) => element.fmt(f),
                b_contractEvents::ExecuteSwapOrderFilter(element) => element.fmt(f),
                b_contractEvents::InitializeFilter(element) => element.fmt(f),
                b_contractEvents::UpdateDecreaseOrderFilter(element) => element.fmt(f),
                b_contractEvents::UpdateGovFilter(element) => element.fmt(f),
                b_contractEvents::UpdateIncreaseOrderFilter(element) => element.fmt(f),
                b_contractEvents::UpdateMinExecutionFeeFilter(element) => element.fmt(f),
                b_contractEvents::UpdateMinPurchaseTokenAmountUsdFilter(element) => element.fmt(f),
                b_contractEvents::UpdateSwapOrderFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `PRICE_PRECISION` function with signature `PRICE_PRECISION()` and selector `[149, 8, 45, 37]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "PRICE_PRECISION", abi = "PRICE_PRECISION()")]
    pub struct PricePrecisionCall;
    #[doc = "Container type for all input parameters for the `USDG_PRECISION` function with signature `USDG_PRECISION()` and selector `[74, 104, 109, 103]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "USDG_PRECISION", abi = "USDG_PRECISION()")]
    pub struct UsdgPrecisionCall;
    #[doc = "Container type for all input parameters for the `cancelDecreaseOrder` function with signature `cancelDecreaseOrder(uint256)` and selector `[158, 113, 176, 240]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "cancelDecreaseOrder", abi = "cancelDecreaseOrder(uint256)")]
    pub struct CancelDecreaseOrderCall {
        pub order_index: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `cancelIncreaseOrder` function with signature `cancelIncreaseOrder(uint256)` and selector `[71, 224, 187, 208]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "cancelIncreaseOrder", abi = "cancelIncreaseOrder(uint256)")]
    pub struct CancelIncreaseOrderCall {
        pub order_index: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `cancelMultiple` function with signature `cancelMultiple(uint256[],uint256[],uint256[])` and selector `[128, 124, 86, 0]`"]
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
        name = "cancelMultiple",
        abi = "cancelMultiple(uint256[],uint256[],uint256[])"
    )]
    pub struct CancelMultipleCall {
        pub swap_order_indexes: ::std::vec::Vec<ethers::core::types::U256>,
        pub increase_order_indexes: ::std::vec::Vec<ethers::core::types::U256>,
        pub decrease_order_indexes: ::std::vec::Vec<ethers::core::types::U256>,
    }
    #[doc = "Container type for all input parameters for the `cancelSwapOrder` function with signature `cancelSwapOrder(uint256)` and selector `[248, 130, 172, 7]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "cancelSwapOrder", abi = "cancelSwapOrder(uint256)")]
    pub struct CancelSwapOrderCall {
        pub order_index: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `createDecreaseOrder` function with signature `createDecreaseOrder(address,uint256,address,uint256,bool,uint256,bool)` and selector `[193, 108, 222, 138]`"]
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
        name = "createDecreaseOrder",
        abi = "createDecreaseOrder(address,uint256,address,uint256,bool,uint256,bool)"
    )]
    pub struct CreateDecreaseOrderCall {
        pub index_token: ethers::core::types::Address,
        pub size_delta: ethers::core::types::U256,
        pub collateral_token: ethers::core::types::Address,
        pub collateral_delta: ethers::core::types::U256,
        pub is_long: bool,
        pub trigger_price: ethers::core::types::U256,
        pub trigger_above_threshold: bool,
    }
    #[doc = "Container type for all input parameters for the `createIncreaseOrder` function with signature `createIncreaseOrder(address[],uint256,address,uint256,uint256,address,bool,uint256,bool,uint256,bool)` and selector `[177, 66, 164, 176]`"]
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
        name = "createIncreaseOrder",
        abi = "createIncreaseOrder(address[],uint256,address,uint256,uint256,address,bool,uint256,bool,uint256,bool)"
    )]
    pub struct CreateIncreaseOrderCall {
        pub path: ::std::vec::Vec<ethers::core::types::Address>,
        pub amount_in: ethers::core::types::U256,
        pub index_token: ethers::core::types::Address,
        pub min_out: ethers::core::types::U256,
        pub size_delta: ethers::core::types::U256,
        pub collateral_token: ethers::core::types::Address,
        pub is_long: bool,
        pub trigger_price: ethers::core::types::U256,
        pub trigger_above_threshold: bool,
        pub execution_fee: ethers::core::types::U256,
        pub should_wrap: bool,
    }
    #[doc = "Container type for all input parameters for the `createSwapOrder` function with signature `createSwapOrder(address[],uint256,uint256,uint256,bool,uint256,bool,bool)` and selector `[38, 154, 230, 194]`"]
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
        name = "createSwapOrder",
        abi = "createSwapOrder(address[],uint256,uint256,uint256,bool,uint256,bool,bool)"
    )]
    pub struct CreateSwapOrderCall {
        pub path: ::std::vec::Vec<ethers::core::types::Address>,
        pub amount_in: ethers::core::types::U256,
        pub min_out: ethers::core::types::U256,
        pub trigger_ratio: ethers::core::types::U256,
        pub trigger_above_threshold: bool,
        pub execution_fee: ethers::core::types::U256,
        pub should_wrap: bool,
        pub should_unwrap: bool,
    }
    #[doc = "Container type for all input parameters for the `decreaseOrders` function with signature `decreaseOrders(address,uint256)` and selector `[242, 210, 224, 27]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "decreaseOrders", abi = "decreaseOrders(address,uint256)")]
    pub struct DecreaseOrdersCall(
        pub ethers::core::types::Address,
        pub ethers::core::types::U256,
    );
    #[doc = "Container type for all input parameters for the `decreaseOrdersIndex` function with signature `decreaseOrdersIndex(address)` and selector `[213, 102, 208, 202]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "decreaseOrdersIndex", abi = "decreaseOrdersIndex(address)")]
    pub struct DecreaseOrdersIndexCall(pub ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `executeDecreaseOrder` function with signature `executeDecreaseOrder(address,uint256,address)` and selector `[17, 217, 68, 74]`"]
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
        name = "executeDecreaseOrder",
        abi = "executeDecreaseOrder(address,uint256,address)"
    )]
    pub struct ExecuteDecreaseOrderCall {
        pub address: ethers::core::types::Address,
        pub order_index: ethers::core::types::U256,
        pub fee_receiver: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `executeIncreaseOrder` function with signature `executeIncreaseOrder(address,uint256,address)` and selector `[211, 138, 181, 25]`"]
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
        name = "executeIncreaseOrder",
        abi = "executeIncreaseOrder(address,uint256,address)"
    )]
    pub struct ExecuteIncreaseOrderCall {
        pub address: ethers::core::types::Address,
        pub order_index: ethers::core::types::U256,
        pub fee_receiver: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `executeSwapOrder` function with signature `executeSwapOrder(address,uint256,address)` and selector `[7, 199, 237, 195]`"]
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
        name = "executeSwapOrder",
        abi = "executeSwapOrder(address,uint256,address)"
    )]
    pub struct ExecuteSwapOrderCall {
        pub account: ethers::core::types::Address,
        pub order_index: ethers::core::types::U256,
        pub fee_receiver: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getDecreaseOrder` function with signature `getDecreaseOrder(address,uint256)` and selector `[2, 96, 50, 238]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getDecreaseOrder", abi = "getDecreaseOrder(address,uint256)")]
    pub struct GetDecreaseOrderCall {
        pub account: ethers::core::types::Address,
        pub order_index: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `getIncreaseOrder` function with signature `getIncreaseOrder(address,uint256)` and selector `[211, 186, 177, 209]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getIncreaseOrder", abi = "getIncreaseOrder(address,uint256)")]
    pub struct GetIncreaseOrderCall {
        pub account: ethers::core::types::Address,
        pub order_index: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `getSwapOrder` function with signature `getSwapOrder(address,uint256)` and selector `[208, 212, 12, 214]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getSwapOrder", abi = "getSwapOrder(address,uint256)")]
    pub struct GetSwapOrderCall {
        pub account: ethers::core::types::Address,
        pub order_index: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `getUsdgMinPrice` function with signature `getUsdgMinPrice(address)` and selector `[158, 35, 222, 92]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getUsdgMinPrice", abi = "getUsdgMinPrice(address)")]
    pub struct GetUsdgMinPriceCall {
        pub other_token: ethers::core::types::Address,
    }
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
    #[doc = "Container type for all input parameters for the `increaseOrders` function with signature `increaseOrders(address,uint256)` and selector `[43, 125, 98, 144]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "increaseOrders", abi = "increaseOrders(address,uint256)")]
    pub struct IncreaseOrdersCall(
        pub ethers::core::types::Address,
        pub ethers::core::types::U256,
    );
    #[doc = "Container type for all input parameters for the `increaseOrdersIndex` function with signature `increaseOrdersIndex(address)` and selector `[174, 194, 36, 85]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "increaseOrdersIndex", abi = "increaseOrdersIndex(address)")]
    pub struct IncreaseOrdersIndexCall(pub ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `initialize` function with signature `initialize(address,address,address,address,uint256,uint256)` and selector `[215, 196, 28, 121]`"]
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
        name = "initialize",
        abi = "initialize(address,address,address,address,uint256,uint256)"
    )]
    pub struct InitializeCall {
        pub router: ethers::core::types::Address,
        pub vault: ethers::core::types::Address,
        pub weth: ethers::core::types::Address,
        pub usdg: ethers::core::types::Address,
        pub min_execution_fee: ethers::core::types::U256,
        pub min_purchase_token_amount_usd: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `isInitialized` function with signature `isInitialized()` and selector `[57, 46, 83, 205]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "isInitialized", abi = "isInitialized()")]
    pub struct IsInitializedCall;
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
    #[doc = "Container type for all input parameters for the `minPurchaseTokenAmountUsd` function with signature `minPurchaseTokenAmountUsd()` and selector `[141, 225, 12, 46]`"]
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
        name = "minPurchaseTokenAmountUsd",
        abi = "minPurchaseTokenAmountUsd()"
    )]
    pub struct MinPurchaseTokenAmountUsdCall;
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
    #[doc = "Container type for all input parameters for the `setMinPurchaseTokenAmountUsd` function with signature `setMinPurchaseTokenAmountUsd(uint256)` and selector `[13, 92, 201, 56]`"]
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
        name = "setMinPurchaseTokenAmountUsd",
        abi = "setMinPurchaseTokenAmountUsd(uint256)"
    )]
    pub struct SetMinPurchaseTokenAmountUsdCall {
        pub min_purchase_token_amount_usd: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `swapOrders` function with signature `swapOrders(address,uint256)` and selector `[121, 34, 31, 162]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "swapOrders", abi = "swapOrders(address,uint256)")]
    pub struct SwapOrdersCall(
        pub ethers::core::types::Address,
        pub ethers::core::types::U256,
    );
    #[doc = "Container type for all input parameters for the `swapOrdersIndex` function with signature `swapOrdersIndex(address)` and selector `[0, 207, 6, 107]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "swapOrdersIndex", abi = "swapOrdersIndex(address)")]
    pub struct SwapOrdersIndexCall(pub ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `updateDecreaseOrder` function with signature `updateDecreaseOrder(uint256,uint256,uint256,uint256,bool)` and selector `[163, 151, 234, 84]`"]
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
        name = "updateDecreaseOrder",
        abi = "updateDecreaseOrder(uint256,uint256,uint256,uint256,bool)"
    )]
    pub struct UpdateDecreaseOrderCall {
        pub order_index: ethers::core::types::U256,
        pub collateral_delta: ethers::core::types::U256,
        pub size_delta: ethers::core::types::U256,
        pub trigger_price: ethers::core::types::U256,
        pub trigger_above_threshold: bool,
    }
    #[doc = "Container type for all input parameters for the `updateIncreaseOrder` function with signature `updateIncreaseOrder(uint256,uint256,uint256,bool)` and selector `[153, 131, 238, 27]`"]
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
        name = "updateIncreaseOrder",
        abi = "updateIncreaseOrder(uint256,uint256,uint256,bool)"
    )]
    pub struct UpdateIncreaseOrderCall {
        pub order_index: ethers::core::types::U256,
        pub size_delta: ethers::core::types::U256,
        pub trigger_price: ethers::core::types::U256,
        pub trigger_above_threshold: bool,
    }
    #[doc = "Container type for all input parameters for the `updateSwapOrder` function with signature `updateSwapOrder(uint256,uint256,uint256,bool)` and selector `[200, 107, 15, 125]`"]
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
        name = "updateSwapOrder",
        abi = "updateSwapOrder(uint256,uint256,uint256,bool)"
    )]
    pub struct UpdateSwapOrderCall {
        pub order_index: ethers::core::types::U256,
        pub min_out: ethers::core::types::U256,
        pub trigger_ratio: ethers::core::types::U256,
        pub trigger_above_threshold: bool,
    }
    #[doc = "Container type for all input parameters for the `usdg` function with signature `usdg()` and selector `[245, 185, 27, 123]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "usdg", abi = "usdg()")]
    pub struct UsdgCall;
    #[doc = "Container type for all input parameters for the `validatePositionOrderPrice` function with signature `validatePositionOrderPrice(bool,uint256,address,bool,bool)` and selector `[76, 84, 240, 176]`"]
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
        name = "validatePositionOrderPrice",
        abi = "validatePositionOrderPrice(bool,uint256,address,bool,bool)"
    )]
    pub struct ValidatePositionOrderPriceCall {
        pub trigger_above_threshold: bool,
        pub trigger_price: ethers::core::types::U256,
        pub index_token: ethers::core::types::Address,
        pub maximize_price: bool,
        pub raise: bool,
    }
    #[doc = "Container type for all input parameters for the `validateSwapOrderPriceWithTriggerAboveThreshold` function with signature `validateSwapOrderPriceWithTriggerAboveThreshold(address[],uint256)` and selector `[196, 161, 130, 27]`"]
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
        name = "validateSwapOrderPriceWithTriggerAboveThreshold",
        abi = "validateSwapOrderPriceWithTriggerAboveThreshold(address[],uint256)"
    )]
    pub struct ValidateSwapOrderPriceWithTriggerAboveThresholdCall {
        pub path: ::std::vec::Vec<ethers::core::types::Address>,
        pub trigger_ratio: ethers::core::types::U256,
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
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum b_contractCalls {
        PricePrecision(PricePrecisionCall),
        UsdgPrecision(UsdgPrecisionCall),
        CancelDecreaseOrder(CancelDecreaseOrderCall),
        CancelIncreaseOrder(CancelIncreaseOrderCall),
        CancelMultiple(CancelMultipleCall),
        CancelSwapOrder(CancelSwapOrderCall),
        CreateDecreaseOrder(CreateDecreaseOrderCall),
        CreateIncreaseOrder(CreateIncreaseOrderCall),
        CreateSwapOrder(CreateSwapOrderCall),
        DecreaseOrders(DecreaseOrdersCall),
        DecreaseOrdersIndex(DecreaseOrdersIndexCall),
        ExecuteDecreaseOrder(ExecuteDecreaseOrderCall),
        ExecuteIncreaseOrder(ExecuteIncreaseOrderCall),
        ExecuteSwapOrder(ExecuteSwapOrderCall),
        GetDecreaseOrder(GetDecreaseOrderCall),
        GetIncreaseOrder(GetIncreaseOrderCall),
        GetSwapOrder(GetSwapOrderCall),
        GetUsdgMinPrice(GetUsdgMinPriceCall),
        Gov(GovCall),
        IncreaseOrders(IncreaseOrdersCall),
        IncreaseOrdersIndex(IncreaseOrdersIndexCall),
        Initialize(InitializeCall),
        IsInitialized(IsInitializedCall),
        MinExecutionFee(MinExecutionFeeCall),
        MinPurchaseTokenAmountUsd(MinPurchaseTokenAmountUsdCall),
        Router(RouterCall),
        SetGov(SetGovCall),
        SetMinExecutionFee(SetMinExecutionFeeCall),
        SetMinPurchaseTokenAmountUsd(SetMinPurchaseTokenAmountUsdCall),
        SwapOrders(SwapOrdersCall),
        SwapOrdersIndex(SwapOrdersIndexCall),
        UpdateDecreaseOrder(UpdateDecreaseOrderCall),
        UpdateIncreaseOrder(UpdateIncreaseOrderCall),
        UpdateSwapOrder(UpdateSwapOrderCall),
        Usdg(UsdgCall),
        ValidatePositionOrderPrice(ValidatePositionOrderPriceCall),
        ValidateSwapOrderPriceWithTriggerAboveThreshold(
            ValidateSwapOrderPriceWithTriggerAboveThresholdCall,
        ),
        Vault(VaultCall),
        Weth(WethCall),
    }
    impl ethers::core::abi::AbiDecode for b_contractCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <PricePrecisionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(b_contractCalls::PricePrecision(decoded));
            }
            if let Ok(decoded) =
                <UsdgPrecisionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(b_contractCalls::UsdgPrecision(decoded));
            }
            if let Ok(decoded) =
                <CancelDecreaseOrderCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(b_contractCalls::CancelDecreaseOrder(decoded));
            }
            if let Ok(decoded) =
                <CancelIncreaseOrderCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(b_contractCalls::CancelIncreaseOrder(decoded));
            }
            if let Ok(decoded) =
                <CancelMultipleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(b_contractCalls::CancelMultiple(decoded));
            }
            if let Ok(decoded) =
                <CancelSwapOrderCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(b_contractCalls::CancelSwapOrder(decoded));
            }
            if let Ok(decoded) =
                <CreateDecreaseOrderCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(b_contractCalls::CreateDecreaseOrder(decoded));
            }
            if let Ok(decoded) =
                <CreateIncreaseOrderCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(b_contractCalls::CreateIncreaseOrder(decoded));
            }
            if let Ok(decoded) =
                <CreateSwapOrderCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(b_contractCalls::CreateSwapOrder(decoded));
            }
            if let Ok(decoded) =
                <DecreaseOrdersCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(b_contractCalls::DecreaseOrders(decoded));
            }
            if let Ok(decoded) =
                <DecreaseOrdersIndexCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(b_contractCalls::DecreaseOrdersIndex(decoded));
            }
            if let Ok(decoded) =
                <ExecuteDecreaseOrderCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(b_contractCalls::ExecuteDecreaseOrder(decoded));
            }
            if let Ok(decoded) =
                <ExecuteIncreaseOrderCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(b_contractCalls::ExecuteIncreaseOrder(decoded));
            }
            if let Ok(decoded) =
                <ExecuteSwapOrderCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(b_contractCalls::ExecuteSwapOrder(decoded));
            }
            if let Ok(decoded) =
                <GetDecreaseOrderCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(b_contractCalls::GetDecreaseOrder(decoded));
            }
            if let Ok(decoded) =
                <GetIncreaseOrderCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(b_contractCalls::GetIncreaseOrder(decoded));
            }
            if let Ok(decoded) =
                <GetSwapOrderCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(b_contractCalls::GetSwapOrder(decoded));
            }
            if let Ok(decoded) =
                <GetUsdgMinPriceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(b_contractCalls::GetUsdgMinPrice(decoded));
            }
            if let Ok(decoded) = <GovCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(b_contractCalls::Gov(decoded));
            }
            if let Ok(decoded) =
                <IncreaseOrdersCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(b_contractCalls::IncreaseOrders(decoded));
            }
            if let Ok(decoded) =
                <IncreaseOrdersIndexCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(b_contractCalls::IncreaseOrdersIndex(decoded));
            }
            if let Ok(decoded) =
                <InitializeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(b_contractCalls::Initialize(decoded));
            }
            if let Ok(decoded) =
                <IsInitializedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(b_contractCalls::IsInitialized(decoded));
            }
            if let Ok(decoded) =
                <MinExecutionFeeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(b_contractCalls::MinExecutionFee(decoded));
            }
            if let Ok(decoded) =
                <MinPurchaseTokenAmountUsdCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(b_contractCalls::MinPurchaseTokenAmountUsd(decoded));
            }
            if let Ok(decoded) = <RouterCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(b_contractCalls::Router(decoded));
            }
            if let Ok(decoded) = <SetGovCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(b_contractCalls::SetGov(decoded));
            }
            if let Ok(decoded) =
                <SetMinExecutionFeeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(b_contractCalls::SetMinExecutionFee(decoded));
            }
            if let Ok(decoded) =
                <SetMinPurchaseTokenAmountUsdCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(b_contractCalls::SetMinPurchaseTokenAmountUsd(decoded));
            }
            if let Ok(decoded) =
                <SwapOrdersCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(b_contractCalls::SwapOrders(decoded));
            }
            if let Ok(decoded) =
                <SwapOrdersIndexCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(b_contractCalls::SwapOrdersIndex(decoded));
            }
            if let Ok(decoded) =
                <UpdateDecreaseOrderCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(b_contractCalls::UpdateDecreaseOrder(decoded));
            }
            if let Ok(decoded) =
                <UpdateIncreaseOrderCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(b_contractCalls::UpdateIncreaseOrder(decoded));
            }
            if let Ok(decoded) =
                <UpdateSwapOrderCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(b_contractCalls::UpdateSwapOrder(decoded));
            }
            if let Ok(decoded) = <UsdgCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(b_contractCalls::Usdg(decoded));
            }
            if let Ok(decoded) =
                <ValidatePositionOrderPriceCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(b_contractCalls::ValidatePositionOrderPrice(decoded));
            }
            if let Ok (decoded) = < ValidateSwapOrderPriceWithTriggerAboveThresholdCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (b_contractCalls :: ValidateSwapOrderPriceWithTriggerAboveThreshold (decoded)) }
            if let Ok(decoded) = <VaultCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(b_contractCalls::Vault(decoded));
            }
            if let Ok(decoded) = <WethCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(b_contractCalls::Weth(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for b_contractCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                b_contractCalls::PricePrecision(element) => element.encode(),
                b_contractCalls::UsdgPrecision(element) => element.encode(),
                b_contractCalls::CancelDecreaseOrder(element) => element.encode(),
                b_contractCalls::CancelIncreaseOrder(element) => element.encode(),
                b_contractCalls::CancelMultiple(element) => element.encode(),
                b_contractCalls::CancelSwapOrder(element) => element.encode(),
                b_contractCalls::CreateDecreaseOrder(element) => element.encode(),
                b_contractCalls::CreateIncreaseOrder(element) => element.encode(),
                b_contractCalls::CreateSwapOrder(element) => element.encode(),
                b_contractCalls::DecreaseOrders(element) => element.encode(),
                b_contractCalls::DecreaseOrdersIndex(element) => element.encode(),
                b_contractCalls::ExecuteDecreaseOrder(element) => element.encode(),
                b_contractCalls::ExecuteIncreaseOrder(element) => element.encode(),
                b_contractCalls::ExecuteSwapOrder(element) => element.encode(),
                b_contractCalls::GetDecreaseOrder(element) => element.encode(),
                b_contractCalls::GetIncreaseOrder(element) => element.encode(),
                b_contractCalls::GetSwapOrder(element) => element.encode(),
                b_contractCalls::GetUsdgMinPrice(element) => element.encode(),
                b_contractCalls::Gov(element) => element.encode(),
                b_contractCalls::IncreaseOrders(element) => element.encode(),
                b_contractCalls::IncreaseOrdersIndex(element) => element.encode(),
                b_contractCalls::Initialize(element) => element.encode(),
                b_contractCalls::IsInitialized(element) => element.encode(),
                b_contractCalls::MinExecutionFee(element) => element.encode(),
                b_contractCalls::MinPurchaseTokenAmountUsd(element) => element.encode(),
                b_contractCalls::Router(element) => element.encode(),
                b_contractCalls::SetGov(element) => element.encode(),
                b_contractCalls::SetMinExecutionFee(element) => element.encode(),
                b_contractCalls::SetMinPurchaseTokenAmountUsd(element) => element.encode(),
                b_contractCalls::SwapOrders(element) => element.encode(),
                b_contractCalls::SwapOrdersIndex(element) => element.encode(),
                b_contractCalls::UpdateDecreaseOrder(element) => element.encode(),
                b_contractCalls::UpdateIncreaseOrder(element) => element.encode(),
                b_contractCalls::UpdateSwapOrder(element) => element.encode(),
                b_contractCalls::Usdg(element) => element.encode(),
                b_contractCalls::ValidatePositionOrderPrice(element) => element.encode(),
                b_contractCalls::ValidateSwapOrderPriceWithTriggerAboveThreshold(element) => {
                    element.encode()
                }
                b_contractCalls::Vault(element) => element.encode(),
                b_contractCalls::Weth(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for b_contractCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                b_contractCalls::PricePrecision(element) => element.fmt(f),
                b_contractCalls::UsdgPrecision(element) => element.fmt(f),
                b_contractCalls::CancelDecreaseOrder(element) => element.fmt(f),
                b_contractCalls::CancelIncreaseOrder(element) => element.fmt(f),
                b_contractCalls::CancelMultiple(element) => element.fmt(f),
                b_contractCalls::CancelSwapOrder(element) => element.fmt(f),
                b_contractCalls::CreateDecreaseOrder(element) => element.fmt(f),
                b_contractCalls::CreateIncreaseOrder(element) => element.fmt(f),
                b_contractCalls::CreateSwapOrder(element) => element.fmt(f),
                b_contractCalls::DecreaseOrders(element) => element.fmt(f),
                b_contractCalls::DecreaseOrdersIndex(element) => element.fmt(f),
                b_contractCalls::ExecuteDecreaseOrder(element) => element.fmt(f),
                b_contractCalls::ExecuteIncreaseOrder(element) => element.fmt(f),
                b_contractCalls::ExecuteSwapOrder(element) => element.fmt(f),
                b_contractCalls::GetDecreaseOrder(element) => element.fmt(f),
                b_contractCalls::GetIncreaseOrder(element) => element.fmt(f),
                b_contractCalls::GetSwapOrder(element) => element.fmt(f),
                b_contractCalls::GetUsdgMinPrice(element) => element.fmt(f),
                b_contractCalls::Gov(element) => element.fmt(f),
                b_contractCalls::IncreaseOrders(element) => element.fmt(f),
                b_contractCalls::IncreaseOrdersIndex(element) => element.fmt(f),
                b_contractCalls::Initialize(element) => element.fmt(f),
                b_contractCalls::IsInitialized(element) => element.fmt(f),
                b_contractCalls::MinExecutionFee(element) => element.fmt(f),
                b_contractCalls::MinPurchaseTokenAmountUsd(element) => element.fmt(f),
                b_contractCalls::Router(element) => element.fmt(f),
                b_contractCalls::SetGov(element) => element.fmt(f),
                b_contractCalls::SetMinExecutionFee(element) => element.fmt(f),
                b_contractCalls::SetMinPurchaseTokenAmountUsd(element) => element.fmt(f),
                b_contractCalls::SwapOrders(element) => element.fmt(f),
                b_contractCalls::SwapOrdersIndex(element) => element.fmt(f),
                b_contractCalls::UpdateDecreaseOrder(element) => element.fmt(f),
                b_contractCalls::UpdateIncreaseOrder(element) => element.fmt(f),
                b_contractCalls::UpdateSwapOrder(element) => element.fmt(f),
                b_contractCalls::Usdg(element) => element.fmt(f),
                b_contractCalls::ValidatePositionOrderPrice(element) => element.fmt(f),
                b_contractCalls::ValidateSwapOrderPriceWithTriggerAboveThreshold(element) => {
                    element.fmt(f)
                }
                b_contractCalls::Vault(element) => element.fmt(f),
                b_contractCalls::Weth(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<PricePrecisionCall> for b_contractCalls {
        fn from(var: PricePrecisionCall) -> Self {
            b_contractCalls::PricePrecision(var)
        }
    }
    impl ::std::convert::From<UsdgPrecisionCall> for b_contractCalls {
        fn from(var: UsdgPrecisionCall) -> Self {
            b_contractCalls::UsdgPrecision(var)
        }
    }
    impl ::std::convert::From<CancelDecreaseOrderCall> for b_contractCalls {
        fn from(var: CancelDecreaseOrderCall) -> Self {
            b_contractCalls::CancelDecreaseOrder(var)
        }
    }
    impl ::std::convert::From<CancelIncreaseOrderCall> for b_contractCalls {
        fn from(var: CancelIncreaseOrderCall) -> Self {
            b_contractCalls::CancelIncreaseOrder(var)
        }
    }
    impl ::std::convert::From<CancelMultipleCall> for b_contractCalls {
        fn from(var: CancelMultipleCall) -> Self {
            b_contractCalls::CancelMultiple(var)
        }
    }
    impl ::std::convert::From<CancelSwapOrderCall> for b_contractCalls {
        fn from(var: CancelSwapOrderCall) -> Self {
            b_contractCalls::CancelSwapOrder(var)
        }
    }
    impl ::std::convert::From<CreateDecreaseOrderCall> for b_contractCalls {
        fn from(var: CreateDecreaseOrderCall) -> Self {
            b_contractCalls::CreateDecreaseOrder(var)
        }
    }
    impl ::std::convert::From<CreateIncreaseOrderCall> for b_contractCalls {
        fn from(var: CreateIncreaseOrderCall) -> Self {
            b_contractCalls::CreateIncreaseOrder(var)
        }
    }
    impl ::std::convert::From<CreateSwapOrderCall> for b_contractCalls {
        fn from(var: CreateSwapOrderCall) -> Self {
            b_contractCalls::CreateSwapOrder(var)
        }
    }
    impl ::std::convert::From<DecreaseOrdersCall> for b_contractCalls {
        fn from(var: DecreaseOrdersCall) -> Self {
            b_contractCalls::DecreaseOrders(var)
        }
    }
    impl ::std::convert::From<DecreaseOrdersIndexCall> for b_contractCalls {
        fn from(var: DecreaseOrdersIndexCall) -> Self {
            b_contractCalls::DecreaseOrdersIndex(var)
        }
    }
    impl ::std::convert::From<ExecuteDecreaseOrderCall> for b_contractCalls {
        fn from(var: ExecuteDecreaseOrderCall) -> Self {
            b_contractCalls::ExecuteDecreaseOrder(var)
        }
    }
    impl ::std::convert::From<ExecuteIncreaseOrderCall> for b_contractCalls {
        fn from(var: ExecuteIncreaseOrderCall) -> Self {
            b_contractCalls::ExecuteIncreaseOrder(var)
        }
    }
    impl ::std::convert::From<ExecuteSwapOrderCall> for b_contractCalls {
        fn from(var: ExecuteSwapOrderCall) -> Self {
            b_contractCalls::ExecuteSwapOrder(var)
        }
    }
    impl ::std::convert::From<GetDecreaseOrderCall> for b_contractCalls {
        fn from(var: GetDecreaseOrderCall) -> Self {
            b_contractCalls::GetDecreaseOrder(var)
        }
    }
    impl ::std::convert::From<GetIncreaseOrderCall> for b_contractCalls {
        fn from(var: GetIncreaseOrderCall) -> Self {
            b_contractCalls::GetIncreaseOrder(var)
        }
    }
    impl ::std::convert::From<GetSwapOrderCall> for b_contractCalls {
        fn from(var: GetSwapOrderCall) -> Self {
            b_contractCalls::GetSwapOrder(var)
        }
    }
    impl ::std::convert::From<GetUsdgMinPriceCall> for b_contractCalls {
        fn from(var: GetUsdgMinPriceCall) -> Self {
            b_contractCalls::GetUsdgMinPrice(var)
        }
    }
    impl ::std::convert::From<GovCall> for b_contractCalls {
        fn from(var: GovCall) -> Self {
            b_contractCalls::Gov(var)
        }
    }
    impl ::std::convert::From<IncreaseOrdersCall> for b_contractCalls {
        fn from(var: IncreaseOrdersCall) -> Self {
            b_contractCalls::IncreaseOrders(var)
        }
    }
    impl ::std::convert::From<IncreaseOrdersIndexCall> for b_contractCalls {
        fn from(var: IncreaseOrdersIndexCall) -> Self {
            b_contractCalls::IncreaseOrdersIndex(var)
        }
    }
    impl ::std::convert::From<InitializeCall> for b_contractCalls {
        fn from(var: InitializeCall) -> Self {
            b_contractCalls::Initialize(var)
        }
    }
    impl ::std::convert::From<IsInitializedCall> for b_contractCalls {
        fn from(var: IsInitializedCall) -> Self {
            b_contractCalls::IsInitialized(var)
        }
    }
    impl ::std::convert::From<MinExecutionFeeCall> for b_contractCalls {
        fn from(var: MinExecutionFeeCall) -> Self {
            b_contractCalls::MinExecutionFee(var)
        }
    }
    impl ::std::convert::From<MinPurchaseTokenAmountUsdCall> for b_contractCalls {
        fn from(var: MinPurchaseTokenAmountUsdCall) -> Self {
            b_contractCalls::MinPurchaseTokenAmountUsd(var)
        }
    }
    impl ::std::convert::From<RouterCall> for b_contractCalls {
        fn from(var: RouterCall) -> Self {
            b_contractCalls::Router(var)
        }
    }
    impl ::std::convert::From<SetGovCall> for b_contractCalls {
        fn from(var: SetGovCall) -> Self {
            b_contractCalls::SetGov(var)
        }
    }
    impl ::std::convert::From<SetMinExecutionFeeCall> for b_contractCalls {
        fn from(var: SetMinExecutionFeeCall) -> Self {
            b_contractCalls::SetMinExecutionFee(var)
        }
    }
    impl ::std::convert::From<SetMinPurchaseTokenAmountUsdCall> for b_contractCalls {
        fn from(var: SetMinPurchaseTokenAmountUsdCall) -> Self {
            b_contractCalls::SetMinPurchaseTokenAmountUsd(var)
        }
    }
    impl ::std::convert::From<SwapOrdersCall> for b_contractCalls {
        fn from(var: SwapOrdersCall) -> Self {
            b_contractCalls::SwapOrders(var)
        }
    }
    impl ::std::convert::From<SwapOrdersIndexCall> for b_contractCalls {
        fn from(var: SwapOrdersIndexCall) -> Self {
            b_contractCalls::SwapOrdersIndex(var)
        }
    }
    impl ::std::convert::From<UpdateDecreaseOrderCall> for b_contractCalls {
        fn from(var: UpdateDecreaseOrderCall) -> Self {
            b_contractCalls::UpdateDecreaseOrder(var)
        }
    }
    impl ::std::convert::From<UpdateIncreaseOrderCall> for b_contractCalls {
        fn from(var: UpdateIncreaseOrderCall) -> Self {
            b_contractCalls::UpdateIncreaseOrder(var)
        }
    }
    impl ::std::convert::From<UpdateSwapOrderCall> for b_contractCalls {
        fn from(var: UpdateSwapOrderCall) -> Self {
            b_contractCalls::UpdateSwapOrder(var)
        }
    }
    impl ::std::convert::From<UsdgCall> for b_contractCalls {
        fn from(var: UsdgCall) -> Self {
            b_contractCalls::Usdg(var)
        }
    }
    impl ::std::convert::From<ValidatePositionOrderPriceCall> for b_contractCalls {
        fn from(var: ValidatePositionOrderPriceCall) -> Self {
            b_contractCalls::ValidatePositionOrderPrice(var)
        }
    }
    impl ::std::convert::From<ValidateSwapOrderPriceWithTriggerAboveThresholdCall> for b_contractCalls {
        fn from(var: ValidateSwapOrderPriceWithTriggerAboveThresholdCall) -> Self {
            b_contractCalls::ValidateSwapOrderPriceWithTriggerAboveThreshold(var)
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
    #[doc = "Container type for all return fields from the `PRICE_PRECISION` function with signature `PRICE_PRECISION()` and selector `[149, 8, 45, 37]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct PricePrecisionReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `USDG_PRECISION` function with signature `USDG_PRECISION()` and selector `[74, 104, 109, 103]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct UsdgPrecisionReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `decreaseOrders` function with signature `decreaseOrders(address,uint256)` and selector `[242, 210, 224, 27]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct DecreaseOrdersReturn {
        pub account: ethers::core::types::Address,
        pub collateral_token: ethers::core::types::Address,
        pub collateral_delta: ethers::core::types::U256,
        pub index_token: ethers::core::types::Address,
        pub size_delta: ethers::core::types::U256,
        pub is_long: bool,
        pub trigger_price: ethers::core::types::U256,
        pub trigger_above_threshold: bool,
        pub execution_fee: ethers::core::types::U256,
    }
    #[doc = "Container type for all return fields from the `decreaseOrdersIndex` function with signature `decreaseOrdersIndex(address)` and selector `[213, 102, 208, 202]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct DecreaseOrdersIndexReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `getDecreaseOrder` function with signature `getDecreaseOrder(address,uint256)` and selector `[2, 96, 50, 238]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetDecreaseOrderReturn {
        pub collateral_token: ethers::core::types::Address,
        pub collateral_delta: ethers::core::types::U256,
        pub index_token: ethers::core::types::Address,
        pub size_delta: ethers::core::types::U256,
        pub is_long: bool,
        pub trigger_price: ethers::core::types::U256,
        pub trigger_above_threshold: bool,
        pub execution_fee: ethers::core::types::U256,
    }
    #[doc = "Container type for all return fields from the `getIncreaseOrder` function with signature `getIncreaseOrder(address,uint256)` and selector `[211, 186, 177, 209]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetIncreaseOrderReturn {
        pub purchase_token: ethers::core::types::Address,
        pub purchase_token_amount: ethers::core::types::U256,
        pub collateral_token: ethers::core::types::Address,
        pub index_token: ethers::core::types::Address,
        pub size_delta: ethers::core::types::U256,
        pub is_long: bool,
        pub trigger_price: ethers::core::types::U256,
        pub trigger_above_threshold: bool,
        pub execution_fee: ethers::core::types::U256,
    }
    #[doc = "Container type for all return fields from the `getSwapOrder` function with signature `getSwapOrder(address,uint256)` and selector `[208, 212, 12, 214]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetSwapOrderReturn {
        pub path_0: ethers::core::types::Address,
        pub path_1: ethers::core::types::Address,
        pub path_2: ethers::core::types::Address,
        pub amount_in: ethers::core::types::U256,
        pub min_out: ethers::core::types::U256,
        pub trigger_ratio: ethers::core::types::U256,
        pub trigger_above_threshold: bool,
        pub should_unwrap: bool,
        pub execution_fee: ethers::core::types::U256,
    }
    #[doc = "Container type for all return fields from the `getUsdgMinPrice` function with signature `getUsdgMinPrice(address)` and selector `[158, 35, 222, 92]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetUsdgMinPriceReturn(pub ethers::core::types::U256);
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
    #[doc = "Container type for all return fields from the `increaseOrders` function with signature `increaseOrders(address,uint256)` and selector `[43, 125, 98, 144]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct IncreaseOrdersReturn {
        pub account: ethers::core::types::Address,
        pub purchase_token: ethers::core::types::Address,
        pub purchase_token_amount: ethers::core::types::U256,
        pub collateral_token: ethers::core::types::Address,
        pub index_token: ethers::core::types::Address,
        pub size_delta: ethers::core::types::U256,
        pub is_long: bool,
        pub trigger_price: ethers::core::types::U256,
        pub trigger_above_threshold: bool,
        pub execution_fee: ethers::core::types::U256,
    }
    #[doc = "Container type for all return fields from the `increaseOrdersIndex` function with signature `increaseOrdersIndex(address)` and selector `[174, 194, 36, 85]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct IncreaseOrdersIndexReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `isInitialized` function with signature `isInitialized()` and selector `[57, 46, 83, 205]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct IsInitializedReturn(pub bool);
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
    #[doc = "Container type for all return fields from the `minPurchaseTokenAmountUsd` function with signature `minPurchaseTokenAmountUsd()` and selector `[141, 225, 12, 46]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct MinPurchaseTokenAmountUsdReturn(pub ethers::core::types::U256);
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
    #[doc = "Container type for all return fields from the `swapOrders` function with signature `swapOrders(address,uint256)` and selector `[121, 34, 31, 162]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct SwapOrdersReturn {
        pub account: ethers::core::types::Address,
        pub amount_in: ethers::core::types::U256,
        pub min_out: ethers::core::types::U256,
        pub trigger_ratio: ethers::core::types::U256,
        pub trigger_above_threshold: bool,
        pub should_unwrap: bool,
        pub execution_fee: ethers::core::types::U256,
    }
    #[doc = "Container type for all return fields from the `swapOrdersIndex` function with signature `swapOrdersIndex(address)` and selector `[0, 207, 6, 107]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct SwapOrdersIndexReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `usdg` function with signature `usdg()` and selector `[245, 185, 27, 123]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct UsdgReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `validatePositionOrderPrice` function with signature `validatePositionOrderPrice(bool,uint256,address,bool,bool)` and selector `[76, 84, 240, 176]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct ValidatePositionOrderPriceReturn(pub ethers::core::types::U256, pub bool);
    #[doc = "Container type for all return fields from the `validateSwapOrderPriceWithTriggerAboveThreshold` function with signature `validateSwapOrderPriceWithTriggerAboveThreshold(address[],uint256)` and selector `[196, 161, 130, 27]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct ValidateSwapOrderPriceWithTriggerAboveThresholdReturn(pub bool);
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
