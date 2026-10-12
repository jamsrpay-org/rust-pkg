use jamsrpay_types::currency::{AssetId, Decimals, NetworkId};

use crate::definition::AssetNetworkDefinition as Definition;
use crate::standard::TokenStandard as Standard;

pub(crate) static DEFINITIONS: &[Definition] = &[
    // --------------------------------------------------------
    // Native assets: mainnet
    // --------------------------------------------------------
    Definition::native(AssetId::Trx, NetworkId::Tron, Decimals::SIX),
    Definition::native(AssetId::Bnb, NetworkId::Bsc, Decimals::EIGHTEEN),
    Definition::native(AssetId::Eth, NetworkId::Ethereum, Decimals::EIGHTEEN),
    Definition::native(AssetId::Pol, NetworkId::Polygon, Decimals::EIGHTEEN),
    Definition::native(AssetId::Btc, NetworkId::Bitcoin, Decimals::EIGHT),
    Definition::native(AssetId::Ltc, NetworkId::Litecoin, Decimals::EIGHT),
    Definition::native(AssetId::Sol, NetworkId::Solana, Decimals::NINE),
    // --------------------------------------------------------
    // Native assets: testnet
    // --------------------------------------------------------
    Definition::native(AssetId::Trx, NetworkId::TronNile, Decimals::SIX),
    Definition::native(AssetId::Bnb, NetworkId::BscTestnet, Decimals::EIGHTEEN),
    Definition::native(AssetId::Eth, NetworkId::EthereumSepolia, Decimals::EIGHTEEN),
    Definition::native(AssetId::Pol, NetworkId::PolygonAmoy, Decimals::EIGHTEEN),
    Definition::native(AssetId::Btc, NetworkId::BitcoinTestnet, Decimals::EIGHT),
    Definition::native(AssetId::Ltc, NetworkId::LitecoinTestnet, Decimals::EIGHT),
    Definition::native(AssetId::Sol, NetworkId::SolanaDevnet, Decimals::NINE),
    // --------------------------------------------------------
    // USDT: mainnet
    // --------------------------------------------------------
    Definition::token(
        AssetId::Usdt,
        NetworkId::Tron,
        Decimals::SIX,
        Standard::Trc20,
        "TR7NHqjeKQxGTCi8q8ZY4pL8otSzgjLj6t",
    ),
    Definition::token(
        AssetId::Usdt,
        NetworkId::Ethereum,
        Decimals::SIX,
        Standard::Erc20,
        "0xdAC17F958D2ee523a2206206994597C13D831ec7",
    ),
    Definition::token(
        AssetId::Usdt,
        NetworkId::Bsc,
        Decimals::EIGHTEEN,
        Standard::Bep20,
        "0x55d398326f99059fF775485246999027B3197955",
    ),
    Definition::token(
        AssetId::Usdt,
        NetworkId::Solana,
        Decimals::SIX,
        Standard::Spl,
        "Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB",
    ),
    Definition::token(
        AssetId::Usdt,
        NetworkId::Polygon,
        Decimals::SIX,
        Standard::Erc20,
        "0xc2132D05D31c914a87C6611C10748AEb04B58e8F",
    ),
    // --------------------------------------------------------
    // USDT: testnet
    // --------------------------------------------------------
    Definition::token(
        AssetId::Usdt,
        NetworkId::EthereumSepolia,
        Decimals::SIX,
        Standard::Erc20,
        "0xaA8E23Fb1079EA71e0a56F48a2aA51851D8433D0",
    ),
    Definition::token(
        AssetId::Usdt,
        NetworkId::BscTestnet,
        Decimals::EIGHTEEN,
        Standard::Bep20,
        "0x337610d27c682E347C9cD60BD4b3b107C9d34dDd",
    ),
    Definition::token(
        AssetId::Usdt,
        NetworkId::PolygonAmoy,
        Decimals::SIX,
        Standard::Erc20,
        "0x1616d425Cd540B256475cBfb604586C8598eC0FB",
    ),
    Definition::token(
        AssetId::Usdt,
        NetworkId::SolanaDevnet,
        Decimals::SIX,
        Standard::Spl,
        "CXk2AMBfi3TwaEL2468s6zP8xq9NxTXjp9gjMgzeUynM",
    ),
    // --------------------------------------------------------
    // USDC: mainnet
    // --------------------------------------------------------
    Definition::token(
        AssetId::Usdc,
        NetworkId::Ethereum,
        Decimals::SIX,
        Standard::Erc20,
        "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48",
    ),
    Definition::token(
        AssetId::Usdc,
        NetworkId::Polygon,
        Decimals::SIX,
        Standard::Erc20,
        "0x3c499c542cef5e3811e1192ce70d8cc03d5c3359",
    ),
    Definition::token(
        AssetId::Usdc,
        NetworkId::Bsc,
        Decimals::EIGHTEEN,
        Standard::Bep20,
        "0x8AC76a51cc950d9822D68b83fE1Ad97B32Cd580d",
    ),
    Definition::token(
        AssetId::Usdc,
        NetworkId::Solana,
        Decimals::SIX,
        Standard::Spl,
        "EPjFWdd5AufqSSqeM2q1NzybapC8G4wEGGkZwyTDt1v",
    ),
    // --------------------------------------------------------
    // USDC: testnet
    // --------------------------------------------------------
    Definition::token(
        AssetId::Usdc,
        NetworkId::EthereumSepolia,
        Decimals::SIX,
        Standard::Erc20,
        "0x1c7D4B196Cb0C7B01d743Fbc6116a902379C7238",
    ),
    Definition::token(
        AssetId::Usdc,
        NetworkId::BscTestnet,
        Decimals::EIGHTEEN,
        Standard::Bep20,
        "0x64544969ed7EBf5f083679233325356EbE738930",
    ),
    Definition::token(
        AssetId::Usdc,
        NetworkId::PolygonAmoy,
        Decimals::SIX,
        Standard::Erc20,
        "0x41E94Eb71898E8A90Fe2dC87e92a03C0cf1Fa5AB",
    ),
    Definition::token(
        AssetId::Usdc,
        NetworkId::SolanaDevnet,
        Decimals::SIX,
        Standard::Spl,
        "CXk2AMBfi3TwaEL2468s6zP8xq9NxTXjp9gjMgzeUynM",
    ),
    // --------------------------------------------------------
    // DAI: mainnet
    // --------------------------------------------------------
    Definition::token(
        AssetId::Dai,
        NetworkId::Ethereum,
        Decimals::EIGHTEEN,
        Standard::Erc20,
        "0x6B175474E89094C44Da98b954EedeAC495271d0F",
    ),
    Definition::token(
        AssetId::Dai,
        NetworkId::Bsc,
        Decimals::EIGHTEEN,
        Standard::Bep20,
        "0x1AF3F329e8BE154074D8769D1FFa4eE058B1DBc3",
    ),
    Definition::token(
        AssetId::Dai,
        NetworkId::Polygon,
        Decimals::EIGHTEEN,
        Standard::Erc20,
        "0x8f3Cf7ad23Cd3CaDbD9735AFf958023239c6A063",
    ),
    // --------------------------------------------------------
    // EURC: mainnet
    // --------------------------------------------------------
    Definition::token(
        AssetId::Eurc,
        NetworkId::Ethereum,
        Decimals::SIX,
        Standard::Erc20,
        "0x1aBaEA1f7C830bD89Acc67eC4af516284b1bC33c",
    ),
];
