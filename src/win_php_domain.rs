use serde::de::{IgnoredAny, MapAccess, Visitor};
use serde::{de, Deserialize, Deserializer};
use serde_derive::Deserialize;
use serde_json::Result;
use std::collections::HashMap;
use std::fmt::Formatter;

#[derive(Deserialize, Debug, PartialEq)]
pub struct Download {
    pub path: String,
    pub size: String,
    pub sha256: String,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Build {
    pub mtime: String,
    pub zip: Download,
    pub debug_pack: Download,
    pub devel_pack: Download,
}

#[derive(Debug, PartialEq)]
pub struct Release {
    pub version: String,
    pub builds: HashMap<String, Build>,
}

struct ReleaseVisitor {}

impl<'de> Visitor<'de> for ReleaseVisitor {
    type Value = Release;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("Could not deserialize Release")
    }

    fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut release = Release {
            version: String::from(""),
            builds: Default::default(),
        };

        while let Some(key) = map.next_key::<String>()? {
            if key.contains("ts-") {
                release.builds.insert(key, map.next_value()?);
            } else if key == "version" {
                release.version = map.next_value()?;
            } else {
                map.next_value::<IgnoredAny>()?;
            }
        }

        if release.version.is_empty() {
            return Err(de::Error::missing_field("version"));
        }

        if release.builds.is_empty() {
            return Err(de::Error::missing_field("builds"));
        }

        Ok(release)
    }
}

impl<'de> Deserialize<'de> for Release {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(ReleaseVisitor {})
    }
}

#[derive(Debug, PartialEq)]
pub struct ReleasesResponse {
    pub versions: HashMap<String, Release>,
}

impl ReleasesResponse {
    pub fn from_json(json: &str) -> Result<Self> {
        let result: Result<HashMap<String, Release>> = serde_json::from_str(json);

        if let Err(e) = result {
            return Err(e);
        }

        Ok(Self {
            versions: result.unwrap(),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::win_php_domain::{Build, Download, Release, ReleasesResponse};
    use std::collections::HashMap;

    #[test]
    fn test_parse_releases_response() {
        let data = r#"
            {
                "8.1": {
                    "version": "8.1.17",
                    "ts-vs16-x64": {
                        "mtime": "2023-03-14T23:52:36+01:00",
                        "zip": {
                            "path": "php-8.1.17-Win32-vs16-x64.zip",
                            "size": "29.37MB",
                            "sha256": "e6ab21c9535d8823402eb5266a5d289ee05d0fc4bd72a715b6be29b7990b43d1"
                        },
                        "debug_pack": {
                            "size": "23.85MB",
                            "path": "php-debug-pack-8.1.17-Win32-vs16-x64.zip",
                            "sha256": "46a88cca708595c917266da1e26b1edc95a397c3742b357e7ab2fcb2878f9373"
                        },
                        "devel_pack": {
                            "size": "1.21MB",
                            "path": "php-devel-pack-8.1.17-Win32-vs16-x64.zip",
                            "sha256": "51dcdd07047e3b3bf58acc80d6bb9b36cf7cda68c23f33b0bd0e06fcf95207a6"
                        }
                    },
                    "source": {
                        "path": "php-8.1.17-src.zip",
                        "size": "25.45MB"
                    },
                    "test_pack": {
                        "size": "15.21MB",
                        "path": "php-test-pack-8.1.17.zip",
                        "sha256": "aadbbc32f25bfd7ccd6104b4bb45b2ef8a3674c07b7f8e41d7d21a26fb3c1a8c"
                    },
                    "ts-vs16-x86": {
                        "mtime": "2023-03-14T23:32:40+01:00",
                        "zip": {
                            "path": "php-8.1.17-Win32-vs16-x86.zip",
                            "size": "26.3MB",
                            "sha256": "da1f323ccb32a451d5ade510da492e06bd60783250c7f16407980568ae93d245"
                        },
                        "debug_pack": {
                            "size": "24.06MB",
                            "path": "php-debug-pack-8.1.17-Win32-vs16-x86.zip",
                            "sha256": "a9b620efb347e4dbdcec2b224e354f1047d49d1f1fff69f747113ffb8cf36818"
                        },
                        "devel_pack": {
                            "size": "1.21MB",
                            "path": "php-devel-pack-8.1.17-Win32-vs16-x86.zip",
                            "sha256": "3e5e33c74f17ff21f94ae047c5cff025a2f8652923f045282389c3b165e8063a"
                        }
                    },
                    "nts-vs16-x64": {
                        "mtime": "2023-03-15T00:52:38+01:00",
                        "zip": {
                            "path": "php-8.1.17-nts-Win32-vs16-x64.zip",
                            "size": "29.26MB",
                            "sha256": "63162aff4e103e22dc3526372558811a03ef00a24014444d67a1b644b42f7405"
                        },
                        "debug_pack": {
                            "size": "23.85MB",
                            "path": "php-debug-pack-8.1.17-nts-Win32-vs16-x64.zip",
                            "sha256": "67c2e21a2ac5d8b74c216226673c25be4119011f3ede6b5324d9385f1b006f3d"
                        },
                        "devel_pack": {
                            "size": "1.21MB",
                            "path": "php-devel-pack-8.1.17-nts-Win32-vs16-x64.zip",
                            "sha256": "e6211c41df519f54be6488e0d71edac67d8ce7b1340dcb9ccf84bae54e955e0a"
                        }
                    },
                    "nts-vs16-x86": {
                        "mtime": "2023-03-14T23:34:20+01:00",
                        "zip": {
                            "path": "php-8.1.17-nts-Win32-vs16-x86.zip",
                            "size": "26.32MB",
                            "sha256": "af2bf4a8b8dc5217220831aeb48e77fdba78451c9d1224128eba6d7294f026f1"
                        },
                        "debug_pack": {
                            "size": "24.41MB",
                            "path": "php-debug-pack-8.1.17-nts-Win32-vs16-x86.zip",
                            "sha256": "453006fd9cbe91b0376e4c19a8aef235160556e24e1e08e24d2915274ba12ead"
                        },
                        "devel_pack": {
                            "size": "1.21MB",
                            "path": "php-devel-pack-8.1.17-nts-Win32-vs16-x86.zip",
                            "sha256": "bc87c1eb4162ae57b3b2dd8b926c88e08d27ad9704b2008a63763a9cdfa244d2"
                        }
                    }
                },
                "8.2": {
                    "version": "8.2.4",
                    "ts-vs16-x64": {
                        "mtime": "2023-03-14T18:31:10+01:00",
                        "zip": {
                            "path": "php-8.2.4-Win32-vs16-x64.zip",
                            "size": "30.33MB",
                            "sha256": "a3601fe23adfb4985be52eb3a7365716350e4b857c47583673a9aa53162885a3"
                        },
                        "debug_pack": {
                            "size": "24.51MB",
                            "path": "php-debug-pack-8.2.4-Win32-vs16-x64.zip",
                            "sha256": "931bdb8d04f1c461ffa128c003cdec56d59502cf32ef56e8b4a8466f464ed302"
                        },
                        "devel_pack": {
                            "size": "1.23MB",
                            "path": "php-devel-pack-8.2.4-Win32-vs16-x64.zip",
                            "sha256": "65d9b5aeb821115c9523678daae31490681fb9dd743e426481f9a86d024e4c29"
                        }
                    },
                    "source": {
                        "path": "php-8.2.4-src.zip",
                        "size": "25.95MB"
                    },
                    "test_pack": {
                        "size": "15.48MB",
                        "path": "php-test-pack-8.2.4.zip",
                        "sha256": "40297e5d50ea274f5e65bce40983f9ab0f3330bf3146fe3012fd9d1ad842d724"
                    },
                    "ts-vs16-x86": {
                        "mtime": "2023-03-14T18:22:20+01:00",
                        "zip": {
                            "path": "php-8.2.4-Win32-vs16-x86.zip",
                            "size": "27.1MB",
                            "sha256": "01ec453736bb513b136ebf0c7f12490c593888b1c99d44955d88e4b0b2648b5a"
                        },
                        "debug_pack": {
                            "size": "24.75MB",
                            "path": "php-debug-pack-8.2.4-Win32-vs16-x86.zip",
                            "sha256": "c52dd1c8b48b93cff460543be61776d1a021a42eb0d67194ea4ba043d11336b3"
                        },
                        "devel_pack": {
                            "size": "1.24MB",
                            "path": "php-devel-pack-8.2.4-Win32-vs16-x86.zip",
                            "sha256": "2b1257d26d8503f3b4aea586ddfa905755f75fb34ccb711c5db14c1a3d31a8e3"
                        }
                    },
                    "nts-vs16-x64": {
                        "mtime": "2023-03-14T18:22:42+01:00",
                        "zip": {
                            "path": "php-8.2.4-nts-Win32-vs16-x64.zip",
                            "size": "30.22MB",
                            "sha256": "2a3b323c605cf601405c5f52c77764c6b4f8e3d3f05b25c2c850f3b74a4829fd"
                        },
                        "debug_pack": {
                            "size": "24.49MB",
                            "path": "php-debug-pack-8.2.4-nts-Win32-vs16-x64.zip",
                            "sha256": "c291adff83908b7c99d17c2b2d2ddb85041755425042bec41dd12c8815ae8755"
                        },
                        "devel_pack": {
                            "size": "1.23MB",
                            "path": "php-devel-pack-8.2.4-nts-Win32-vs16-x64.zip",
                            "sha256": "2ce7b2020e196576e6456a7c8aaf9aea7a72fd5acdd71e571bbfe352aee10e60"
                        }
                    },
                    "nts-vs16-x86": {
                        "mtime": "2023-03-14T19:26:40+01:00",
                        "zip": {
                            "path": "php-8.2.4-nts-Win32-vs16-x86.zip",
                            "size": "27.13MB",
                            "sha256": "f452f43a11e10934a03d14a20ac46bf98824a4c95d85db1fbe5c2c95588bda2b"
                        },
                        "debug_pack": {
                            "size": "25.1MB",
                            "path": "php-debug-pack-8.2.4-nts-Win32-vs16-x86.zip",
                            "sha256": "ca29577599b5aaa2cbfdc87acd8dd12208322d7be7d25e5b19f26d70fc4f897f"
                        },
                        "devel_pack": {
                            "size": "1.23MB",
                            "path": "php-devel-pack-8.2.4-nts-Win32-vs16-x86.zip",
                            "sha256": "0f15471ba0d0bdd1e907063ae58623343d3c9fb9b15dbfdb6fe3ba9b3f0b4806"
                        }
                    }
                }
            }"#;

        let mut versions = HashMap::new();
        versions.insert(
            String::from("8.1"),
            Release {
                version: String::from("8.1.17"),
                builds: HashMap::from([(
                    String::from("ts-vs16-x64"),
                    Build {
                        mtime: String::from("2023-03-14T23:52:36+01:00"),
                        zip: Download {
                            path: String::from("php-8.1.17-Win32-vs16-x64.zip"),
                            size: String::from("29.37MB"),
                            sha256: String::from(
                                "e6ab21c9535d8823402eb5266a5d289ee05d0fc4bd72a715b6be29b7990b43d1",
                            ),
                        },
                        debug_pack: Download {
                            path: String::from("php-debug-pack-8.1.17-Win32-vs16-x64.zip"),
                            size: String::from("23.85MB"),
                            sha256: String::from(
                                "46a88cca708595c917266da1e26b1edc95a397c3742b357e7ab2fcb2878f9373",
                            ),
                        },
                        devel_pack: Download {
                            path: String::from("php-devel-pack-8.1.17-Win32-vs16-x64.zip"),
                            size: String::from("1.21MB"),
                            sha256: String::from(
                                "51dcdd07047e3b3bf58acc80d6bb9b36cf7cda68c23f33b0bd0e06fcf95207a6",
                            ),
                        },
                    },
                ),(
                    String::from("ts-vs16-x86"),
                    Build {
                        mtime: String::from("2023-03-14T23:32:40+01:00"),
                        zip: Download {
                            path: String::from("php-8.1.17-Win32-vs16-x86.zip"),
                            size: String::from("26.3MB"),
                            sha256: String::from(
                                "da1f323ccb32a451d5ade510da492e06bd60783250c7f16407980568ae93d245",
                            ),
                        },
                        debug_pack: Download {
                            path: String::from("php-debug-pack-8.1.17-Win32-vs16-x86.zip"),
                            size: String::from("24.06MB"),
                            sha256: String::from(
                                "a9b620efb347e4dbdcec2b224e354f1047d49d1f1fff69f747113ffb8cf36818",
                            ),
                        },
                        devel_pack: Download {
                            path: String::from("php-devel-pack-8.1.17-Win32-vs16-x86.zip"),
                            size: String::from("1.21MB"),
                            sha256: String::from(
                                "3e5e33c74f17ff21f94ae047c5cff025a2f8652923f045282389c3b165e8063a",
                            ),
                        },
                    },
                ),(
                    String::from("nts-vs16-x64"),
                    Build {
                        mtime: String::from("2023-03-15T00:52:38+01:00"),
                        zip: Download {
                            path: String::from("php-8.1.17-nts-Win32-vs16-x64.zip"),
                            size: String::from("29.26MB"),
                            sha256: String::from(
                                "63162aff4e103e22dc3526372558811a03ef00a24014444d67a1b644b42f7405",
                            ),
                        },
                        debug_pack: Download {
                            path: String::from("php-debug-pack-8.1.17-nts-Win32-vs16-x64.zip"),
                            size: String::from("23.85MB"),
                            sha256: String::from(
                                "67c2e21a2ac5d8b74c216226673c25be4119011f3ede6b5324d9385f1b006f3d",
                            ),
                        },
                        devel_pack: Download {
                            path: String::from("php-devel-pack-8.1.17-nts-Win32-vs16-x64.zip"),
                            size: String::from("1.21MB"),
                            sha256: String::from(
                                "e6211c41df519f54be6488e0d71edac67d8ce7b1340dcb9ccf84bae54e955e0a",
                            ),
                        },
                    },
                ),(
                    String::from("nts-vs16-x86"),
                    Build {
                        mtime: String::from("2023-03-14T23:34:20+01:00"),
                        zip: Download {
                            path: String::from("php-8.1.17-nts-Win32-vs16-x86.zip"),
                            size: String::from("26.32MB"),
                            sha256: String::from(
                                "af2bf4a8b8dc5217220831aeb48e77fdba78451c9d1224128eba6d7294f026f1",
                            ),
                        },
                        debug_pack: Download {
                            path: String::from("php-debug-pack-8.1.17-nts-Win32-vs16-x86.zip"),
                            size: String::from("24.41MB"),
                            sha256: String::from(
                                "453006fd9cbe91b0376e4c19a8aef235160556e24e1e08e24d2915274ba12ead",
                            ),
                        },
                        devel_pack: Download {
                            path: String::from("php-devel-pack-8.1.17-nts-Win32-vs16-x86.zip"),
                            size: String::from("1.21MB"),
                            sha256: String::from(
                                "bc87c1eb4162ae57b3b2dd8b926c88e08d27ad9704b2008a63763a9cdfa244d2",
                            ),
                        },
                    },
                )]),
            },
        );
        versions.insert(
            String::from("8.2"),
            Release {
                version: String::from("8.2.4"),
                builds: HashMap::from([(
                    String::from("ts-vs16-x64"),
                    Build {
                        mtime: String::from("2023-03-14T18:31:10+01:00"),
                        zip: Download {
                            path: String::from("php-8.2.4-Win32-vs16-x64.zip"),
                            size: String::from("30.33MB"),
                            sha256: String::from(
                                "a3601fe23adfb4985be52eb3a7365716350e4b857c47583673a9aa53162885a3",
                            ),
                        },
                        debug_pack: Download {
                            path: String::from("php-debug-pack-8.2.4-Win32-vs16-x64.zip"),
                            size: String::from("24.51MB"),
                            sha256: String::from(
                                "931bdb8d04f1c461ffa128c003cdec56d59502cf32ef56e8b4a8466f464ed302",
                            ),
                        },
                        devel_pack: Download {
                            path: String::from("php-devel-pack-8.2.4-Win32-vs16-x64.zip"),
                            size: String::from("1.23MB"),
                            sha256: String::from(
                                "65d9b5aeb821115c9523678daae31490681fb9dd743e426481f9a86d024e4c29",
                            ),
                        },
                    },
                ),(
                    String::from("ts-vs16-x86"),
                    Build {
                        mtime: String::from("2023-03-14T18:22:20+01:00"),
                        zip: Download {
                            path: String::from("php-8.2.4-Win32-vs16-x86.zip"),
                            size: String::from("27.1MB"),
                            sha256: String::from(
                                "01ec453736bb513b136ebf0c7f12490c593888b1c99d44955d88e4b0b2648b5a",
                            ),
                        },
                        debug_pack: Download {
                            path: String::from("php-debug-pack-8.2.4-Win32-vs16-x86.zip"),
                            size: String::from("24.75MB"),
                            sha256: String::from(
                                "c52dd1c8b48b93cff460543be61776d1a021a42eb0d67194ea4ba043d11336b3",
                            ),
                        },
                        devel_pack: Download {
                            path: String::from("php-devel-pack-8.2.4-Win32-vs16-x86.zip"),
                            size: String::from("1.24MB"),
                            sha256: String::from(
                                "2b1257d26d8503f3b4aea586ddfa905755f75fb34ccb711c5db14c1a3d31a8e3",
                            ),
                        },
                    },
                ),(
                    String::from("nts-vs16-x64"),
                    Build {
                        mtime: String::from("2023-03-14T18:22:42+01:00"),
                        zip: Download {
                            path: String::from("php-8.2.4-nts-Win32-vs16-x64.zip"),
                            size: String::from("30.22MB"),
                            sha256: String::from(
                                "2a3b323c605cf601405c5f52c77764c6b4f8e3d3f05b25c2c850f3b74a4829fd",
                            ),
                        },
                        debug_pack: Download {
                            path: String::from("php-debug-pack-8.2.4-nts-Win32-vs16-x64.zip"),
                            size: String::from("24.49MB"),
                            sha256: String::from(
                                "c291adff83908b7c99d17c2b2d2ddb85041755425042bec41dd12c8815ae8755",
                            ),
                        },
                        devel_pack: Download {
                            path: String::from("php-devel-pack-8.2.4-nts-Win32-vs16-x64.zip"),
                            size: String::from("1.23MB"),
                            sha256: String::from(
                                "2ce7b2020e196576e6456a7c8aaf9aea7a72fd5acdd71e571bbfe352aee10e60",
                            ),
                        },
                    },
                ),(
                    String::from("nts-vs16-x86"),
                    Build {
                        mtime: String::from("2023-03-14T19:26:40+01:00"),
                        zip: Download {
                            path: String::from("php-8.2.4-nts-Win32-vs16-x86.zip"),
                            size: String::from("27.13MB"),
                            sha256: String::from(
                                "f452f43a11e10934a03d14a20ac46bf98824a4c95d85db1fbe5c2c95588bda2b",
                            ),
                        },
                        debug_pack: Download {
                            path: String::from("php-debug-pack-8.2.4-nts-Win32-vs16-x86.zip"),
                            size: String::from("25.1MB"),
                            sha256: String::from(
                                "ca29577599b5aaa2cbfdc87acd8dd12208322d7be7d25e5b19f26d70fc4f897f",
                            ),
                        },
                        devel_pack: Download {
                            path: String::from("php-devel-pack-8.2.4-nts-Win32-vs16-x86.zip"),
                            size: String::from("1.23MB"),
                            sha256: String::from(
                                "0f15471ba0d0bdd1e907063ae58623343d3c9fb9b15dbfdb6fe3ba9b3f0b4806",
                            ),
                        },
                    },
                )]),
            },
        );

        assert_eq!(
            ReleasesResponse { versions },
            ReleasesResponse::from_json(data).unwrap()
        )
    }
}
