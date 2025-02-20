#[cfg(feature = "parachain")]
use crate::chain_spec::get_from_seed;
use crate::chain_spec::{
    generic_genesis, get_account_id_from_seed, zeitgeist_wasm, AdditionalChainSpec, ChainSpec,
};
use sc_service::ChainType;
use sp_core::sr25519;

pub fn dev_config(
    #[cfg(feature = "parachain")] parachain_id: cumulus_primitives_core::ParaId,
) -> Result<ChainSpec, String> {
    let zeitgeist_wasm = zeitgeist_wasm()?;
    Ok(ChainSpec::from_genesis(
        "Development",
        "dev",
        ChainType::Local,
        move || {
            generic_genesis(
                #[cfg(feature = "parachain")]
                AdditionalChainSpec {
                    candidates: vec![(
                        get_account_id_from_seed::<sr25519::Public>("Alice"),
                        get_from_seed::<nimbus_primitives::NimbusId>("Alice"),
                        crate::chain_spec::DEFAULT_STAKING_AMOUNT,
                    )],
                    inflation_info: crate::chain_spec::DEFAULT_COLLATOR_INFLATION_INFO,
                    nominations: vec![],
                    parachain_id,
                },
                #[cfg(not(feature = "parachain"))]
                AdditionalChainSpec {
                    initial_authorities: vec![crate::chain_spec::authority_keys_from_seed("Alice")],
                },
                vec![
                    get_account_id_from_seed::<sr25519::Public>("Alice"),
                    get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Bob"),
                    get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Charlie"),
                    get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Dave"),
                    get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Eve"),
                    get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Ferdie"),
                    get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
                ],
                zeitgeist_primitives::types::Balance::MAX >> 4,
                get_account_id_from_seed::<sr25519::Public>("Alice"),
                zeitgeist_wasm,
            )
        },
        vec![],
        None,
        None,
        None,
        #[cfg(feature = "parachain")]
        crate::chain_spec::Extensions {
            relay_chain: "rococo-local".into(),
            parachain_id: parachain_id.into(),
        },
        #[cfg(not(feature = "parachain"))]
        Default::default(),
    ))
}
