use hgs_core::{
    enums::Pull,
    settings::{Pool, PoolContents, PoolRef, Settings, SoftPitySettings},
};

#[test]
fn test_simple_valid_settings() {
    let subject = Settings::new(vec![Pool {
        name: "some pool"
        rate: 1.0,
        contents: PoolContents::Pulls(vec![Pull::Character(1)]),

        soft_pity_settings: None,
        hard_pity: None,
        guaranteed_pool: None,
    }]);

    assert!(subject.is_ok());
}

#[test]
fn test_complex_valid_settings() {
    // this should look similar to Character Banner
    let subject = Settings::new(vec![
        Pool {
            name: "5*"
            rate: 0.6,
            contents: PoolContents::SubPools(vec![
                Pool {
                    // limited character
                    name: "Banner",
                    rate: 0.55,
                    contents: PoolContents::Pulls(vec![Pull::Character(1)]),
                    soft_pity_settings: None,
                    hard_pity: None,
                    guaranteed_pool: None,
                },
                Pool {
                    // standard characters
                    name: "Standard",
                    rate: 0.45,
                    contents: PoolContents::Pulls(vec![
                        Pull::Character(2),
                        Pull::Character(4),
                        Pull::Character(4),
                    ]),
                    soft_pity_settings: None,
                    hard_pity: None,
                },
            ]),
            soft_pity_settings: Some(SoftPitySettings {
                pity_start: 74,
                rate_increase: 0.06,
            }),
            hard_pity: Some(90),
            guaranteed_pool: Some(PoolRef::ByName("Banner"))
        },
        Pool {
            rate: 0.051,
            contents: PoolContents::SubPools(vec![
                Pool {
                    rate: 0.5,
                    contents: PoolContents::SubPools(vec![
                        Pool {
                            // banner 4*s
                            rate: 0.5,
                            contents: PoolContents::Pulls(vec![
                                Pull::Character(41),
                                Pull::Character(42),
                                Pull::Character(43),
                            ]),

                            soft_pity_settings: None,
                            hard_pity: None,
                            guarantee_enabled: true,
                        },
                        Pool {
                            rate: 0.5,
                            contents: PoolContents::Pulls(vec![
                                Pull::Character(44),
                                Pull::Character(45),
                                Pull::Character(46),
                            ]),

                            soft_pity_settings: None,
                            hard_pity: None,
                            guarantee_enabled: false,
                        }
                    ]),

                    soft_pity_settings: None,
                    hard_pity: None,
                    guarantee_enabled: true,
                },
                Pool {
                    rate: 0.5,
                    contents: PoolContents::Pulls(vec![
                        Pull::LightCone(1),
                        Pull::LightCone(2),
                        Pull::LightCone(3),
                        Pull::LightCone(4),
                    ]),

                    soft_pity_settings: None,
                    hard_pity: None,
                    guarantee_enabled: false,
                },
            ]),
            soft_pity_settings: None,
            hard_pity: Some(10),
            guarantee_enabled: false,
        },
    ]);

    assert!(subject.is_ok());
}

// invalid settings only now...
#[test]
fn test_only_one_guaranteed_in_branch() {
    // only one sibling can be guaranteed at a time, does not impact any
    // parent or child
}