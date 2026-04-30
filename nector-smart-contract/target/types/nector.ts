/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/nector.json`.
 */
export type Nector = {
  "address": "9buv2Wao6udvrauoot3JuGGJ8cB3ULfX2r1BzVNw1h64",
  "metadata": {
    "name": "nector",
    "version": "0.1.0",
    "spec": "0.1.0",
    "description": "Created with Anchor"
  },
  "instructions": [
    {
      "name": "buyerCancel",
      "discriminator": [
        110,
        82,
        216,
        227,
        40,
        38,
        49,
        69
      ],
      "accounts": [
        {
          "name": "order",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  111,
                  114,
                  100,
                  101,
                  114
                ]
              },
              {
                "kind": "account",
                "path": "seller"
              },
              {
                "kind": "arg",
                "path": "orderIndex"
              }
            ]
          }
        },
        {
          "name": "escrow",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  101,
                  115,
                  99,
                  114,
                  111,
                  119
                ]
              },
              {
                "kind": "account",
                "path": "order"
              }
            ]
          }
        },
        {
          "name": "buyer",
          "writable": true,
          "signer": true
        },
        {
          "name": "seller",
          "docs": [
            "CHECK"
          ]
        }
      ],
      "args": [
        {
          "name": "orderIndex",
          "type": "u64"
        }
      ]
    },
    {
      "name": "buyerFundEscrow",
      "discriminator": [
        202,
        43,
        2,
        1,
        169,
        40,
        13,
        214
      ],
      "accounts": [
        {
          "name": "order",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  111,
                  114,
                  100,
                  101,
                  114
                ]
              },
              {
                "kind": "account",
                "path": "seller"
              },
              {
                "kind": "arg",
                "path": "orderIndex"
              }
            ]
          }
        },
        {
          "name": "escrow",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  101,
                  115,
                  99,
                  114,
                  111,
                  119
                ]
              },
              {
                "kind": "account",
                "path": "order"
              }
            ]
          }
        },
        {
          "name": "buyer",
          "writable": true,
          "signer": true
        },
        {
          "name": "seller"
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        },
        {
          "name": "feeWallet",
          "writable": true
        }
      ],
      "args": [
        {
          "name": "orderIndex",
          "type": "u64"
        }
      ]
    },
    {
      "name": "buyerWin",
      "discriminator": [
        26,
        166,
        53,
        87,
        183,
        7,
        84,
        217
      ],
      "accounts": [
        {
          "name": "order",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  111,
                  114,
                  100,
                  101,
                  114
                ]
              },
              {
                "kind": "account",
                "path": "seller"
              },
              {
                "kind": "arg",
                "path": "orderIndex"
              }
            ]
          }
        },
        {
          "name": "escrow",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  101,
                  115,
                  99,
                  114,
                  111,
                  119
                ]
              },
              {
                "kind": "account",
                "path": "order"
              }
            ]
          }
        },
        {
          "name": "buyer",
          "writable": true
        },
        {
          "name": "seller",
          "writable": true
        },
        {
          "name": "feeWallet",
          "writable": true
        }
      ],
      "args": [
        {
          "name": "orderIndex",
          "type": "u64"
        }
      ]
    },
    {
      "name": "confirmDelivery",
      "discriminator": [
        11,
        109,
        227,
        53,
        179,
        190,
        88,
        155
      ],
      "accounts": [
        {
          "name": "order",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  111,
                  114,
                  100,
                  101,
                  114
                ]
              },
              {
                "kind": "account",
                "path": "seller"
              },
              {
                "kind": "arg",
                "path": "orderIndex"
              }
            ]
          }
        },
        {
          "name": "escrow",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  101,
                  115,
                  99,
                  114,
                  111,
                  119
                ]
              },
              {
                "kind": "account",
                "path": "order"
              }
            ]
          }
        },
        {
          "name": "buyer",
          "writable": true,
          "signer": true
        },
        {
          "name": "seller",
          "docs": [
            "CHECK"
          ],
          "writable": true
        }
      ],
      "args": [
        {
          "name": "orderIndex",
          "type": "u64"
        }
      ]
    },
    {
      "name": "confirmTimeout",
      "discriminator": [
        76,
        25,
        243,
        112,
        135,
        211,
        120,
        215
      ],
      "accounts": [
        {
          "name": "order",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  111,
                  114,
                  100,
                  101,
                  114
                ]
              },
              {
                "kind": "account",
                "path": "seller"
              },
              {
                "kind": "arg",
                "path": "orderIndex"
              }
            ]
          }
        },
        {
          "name": "escrow",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  101,
                  115,
                  99,
                  114,
                  111,
                  119
                ]
              },
              {
                "kind": "account",
                "path": "order"
              }
            ]
          }
        },
        {
          "name": "buyer",
          "docs": [
            "CHECK"
          ],
          "writable": true
        },
        {
          "name": "seller",
          "docs": [
            "CHECK"
          ],
          "writable": true
        }
      ],
      "args": [
        {
          "name": "orderIndex",
          "type": "u64"
        }
      ]
    },
    {
      "name": "createOrder",
      "discriminator": [
        141,
        54,
        37,
        207,
        237,
        210,
        250,
        215
      ],
      "accounts": [
        {
          "name": "sellerAccount",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  115,
                  101,
                  108,
                  108,
                  101,
                  114
                ]
              },
              {
                "kind": "account",
                "path": "seller"
              }
            ]
          }
        },
        {
          "name": "order",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  111,
                  114,
                  100,
                  101,
                  114
                ]
              },
              {
                "kind": "account",
                "path": "seller"
              },
              {
                "kind": "arg",
                "path": "orderIndex"
              }
            ]
          }
        },
        {
          "name": "seller",
          "writable": true,
          "signer": true
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "orderIndex",
          "type": "u64"
        },
        {
          "name": "mode",
          "type": "u8"
        },
        {
          "name": "productType",
          "type": "u8"
        },
        {
          "name": "orderName",
          "type": "string"
        },
        {
          "name": "buyerWallet",
          "type": "pubkey"
        },
        {
          "name": "priceLamports",
          "type": "u64"
        },
        {
          "name": "shippingHours",
          "type": "u32"
        }
      ]
    },
    {
      "name": "draw",
      "discriminator": [
        61,
        40,
        62,
        184,
        31,
        176,
        24,
        130
      ],
      "accounts": [
        {
          "name": "order",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  111,
                  114,
                  100,
                  101,
                  114
                ]
              },
              {
                "kind": "account",
                "path": "seller"
              },
              {
                "kind": "arg",
                "path": "orderIndex"
              }
            ]
          }
        },
        {
          "name": "escrow",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  101,
                  115,
                  99,
                  114,
                  111,
                  119
                ]
              },
              {
                "kind": "account",
                "path": "order"
              }
            ]
          }
        },
        {
          "name": "buyer",
          "docs": [
            "CHECK"
          ],
          "writable": true
        },
        {
          "name": "seller",
          "docs": [
            "CHECK"
          ],
          "writable": true
        },
        {
          "name": "feeWallet",
          "docs": [
            "CHECK"
          ],
          "writable": true
        }
      ],
      "args": [
        {
          "name": "orderIndex",
          "type": "u64"
        }
      ]
    },
    {
      "name": "initSeller",
      "discriminator": [
        76,
        68,
        121,
        218,
        196,
        191,
        183,
        169
      ],
      "accounts": [
        {
          "name": "sellerAccount",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  115,
                  101,
                  108,
                  108,
                  101,
                  114
                ]
              },
              {
                "kind": "account",
                "path": "seller"
              }
            ]
          }
        },
        {
          "name": "seller",
          "writable": true,
          "signer": true
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": []
    },
    {
      "name": "markShipped",
      "discriminator": [
        239,
        5,
        66,
        105,
        238,
        17,
        89,
        97
      ],
      "accounts": [
        {
          "name": "order",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  111,
                  114,
                  100,
                  101,
                  114
                ]
              },
              {
                "kind": "account",
                "path": "seller"
              },
              {
                "kind": "arg",
                "path": "orderIndex"
              }
            ]
          }
        },
        {
          "name": "seller",
          "writable": true,
          "signer": true
        }
      ],
      "args": [
        {
          "name": "orderIndex",
          "type": "u64"
        }
      ]
    },
    {
      "name": "openDispute",
      "discriminator": [
        137,
        25,
        99,
        119,
        23,
        223,
        161,
        42
      ],
      "accounts": [
        {
          "name": "order",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  111,
                  114,
                  100,
                  101,
                  114
                ]
              },
              {
                "kind": "account",
                "path": "seller"
              },
              {
                "kind": "arg",
                "path": "orderIndex"
              }
            ]
          }
        },
        {
          "name": "buyer",
          "writable": true,
          "signer": true
        },
        {
          "name": "seller",
          "docs": [
            "CHECK"
          ]
        }
      ],
      "args": [
        {
          "name": "orderIndex",
          "type": "u64"
        }
      ]
    },
    {
      "name": "paySellerDuringDiscuss",
      "discriminator": [
        73,
        35,
        232,
        153,
        200,
        12,
        146,
        143
      ],
      "accounts": [
        {
          "name": "order",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  111,
                  114,
                  100,
                  101,
                  114
                ]
              },
              {
                "kind": "account",
                "path": "order.seller_wallet",
                "account": "order"
              },
              {
                "kind": "arg",
                "path": "orderIndex"
              }
            ]
          }
        },
        {
          "name": "escrow",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  101,
                  115,
                  99,
                  114,
                  111,
                  119
                ]
              },
              {
                "kind": "account",
                "path": "order"
              }
            ]
          }
        },
        {
          "name": "buyer",
          "writable": true,
          "signer": true
        },
        {
          "name": "seller",
          "docs": [
            "CHECK"
          ],
          "writable": true
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "orderIndex",
          "type": "u64"
        }
      ]
    },
    {
      "name": "refundBuyer",
      "discriminator": [
        199,
        139,
        203,
        146,
        192,
        150,
        53,
        218
      ],
      "accounts": [
        {
          "name": "order",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  111,
                  114,
                  100,
                  101,
                  114
                ]
              },
              {
                "kind": "account",
                "path": "seller"
              },
              {
                "kind": "arg",
                "path": "orderIndex"
              }
            ]
          }
        },
        {
          "name": "escrow",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  101,
                  115,
                  99,
                  114,
                  111,
                  119
                ]
              },
              {
                "kind": "account",
                "path": "order"
              }
            ]
          }
        },
        {
          "name": "seller",
          "writable": true,
          "signer": true
        },
        {
          "name": "buyer",
          "docs": [
            "CHECK"
          ],
          "writable": true
        }
      ],
      "args": [
        {
          "name": "orderIndex",
          "type": "u64"
        }
      ]
    },
    {
      "name": "refundDuringDiscuss",
      "discriminator": [
        194,
        61,
        173,
        133,
        118,
        5,
        188,
        52
      ],
      "accounts": [
        {
          "name": "order",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  111,
                  114,
                  100,
                  101,
                  114
                ]
              },
              {
                "kind": "account",
                "path": "order.seller_wallet",
                "account": "order"
              },
              {
                "kind": "arg",
                "path": "orderIndex"
              }
            ]
          }
        },
        {
          "name": "escrow",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  101,
                  115,
                  99,
                  114,
                  111,
                  119
                ]
              },
              {
                "kind": "account",
                "path": "order"
              }
            ]
          }
        },
        {
          "name": "buyer",
          "docs": [
            "CHECK"
          ],
          "writable": true
        },
        {
          "name": "seller",
          "writable": true,
          "signer": true
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "orderIndex",
          "type": "u64"
        }
      ]
    },
    {
      "name": "respondDispute",
      "discriminator": [
        71,
        136,
        87,
        127,
        213,
        117,
        241,
        1
      ],
      "accounts": [
        {
          "name": "order",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  111,
                  114,
                  100,
                  101,
                  114
                ]
              },
              {
                "kind": "account",
                "path": "seller"
              },
              {
                "kind": "arg",
                "path": "orderIndex"
              }
            ]
          }
        },
        {
          "name": "seller",
          "writable": true,
          "signer": true
        }
      ],
      "args": [
        {
          "name": "orderIndex",
          "type": "u64"
        }
      ]
    },
    {
      "name": "sellerCancel",
      "discriminator": [
        32,
        135,
        125,
        225,
        9,
        49,
        24,
        176
      ],
      "accounts": [
        {
          "name": "order",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  111,
                  114,
                  100,
                  101,
                  114
                ]
              },
              {
                "kind": "account",
                "path": "order.seller_wallet",
                "account": "order"
              },
              {
                "kind": "arg",
                "path": "orderIndex"
              }
            ]
          }
        },
        {
          "name": "escrow",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  101,
                  115,
                  99,
                  114,
                  111,
                  119
                ]
              },
              {
                "kind": "account",
                "path": "order"
              }
            ]
          }
        },
        {
          "name": "buyer",
          "docs": [
            "CHECK"
          ],
          "writable": true
        },
        {
          "name": "seller",
          "writable": true,
          "signer": true
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "orderIndex",
          "type": "u64"
        }
      ]
    },
    {
      "name": "sellerFundEscrow",
      "discriminator": [
        144,
        79,
        130,
        127,
        151,
        54,
        246,
        18
      ],
      "accounts": [
        {
          "name": "order",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  111,
                  114,
                  100,
                  101,
                  114
                ]
              },
              {
                "kind": "account",
                "path": "seller"
              },
              {
                "kind": "arg",
                "path": "orderIndex"
              }
            ]
          }
        },
        {
          "name": "escrow",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  101,
                  115,
                  99,
                  114,
                  111,
                  119
                ]
              },
              {
                "kind": "account",
                "path": "order"
              }
            ]
          }
        },
        {
          "name": "seller",
          "writable": true,
          "signer": true
        },
        {
          "name": "feeWallet",
          "writable": true
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "orderIndex",
          "type": "u64"
        }
      ]
    },
    {
      "name": "shippingTimeout",
      "discriminator": [
        98,
        192,
        50,
        210,
        239,
        245,
        31,
        99
      ],
      "accounts": [
        {
          "name": "order",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  111,
                  114,
                  100,
                  101,
                  114
                ]
              },
              {
                "kind": "account",
                "path": "seller"
              },
              {
                "kind": "arg",
                "path": "orderIndex"
              }
            ]
          }
        },
        {
          "name": "escrow",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  101,
                  115,
                  99,
                  114,
                  111,
                  119
                ]
              },
              {
                "kind": "account",
                "path": "order"
              }
            ]
          }
        },
        {
          "name": "buyer",
          "docs": [
            "CHECK"
          ],
          "writable": true
        },
        {
          "name": "seller",
          "docs": [
            "CHECK"
          ],
          "writable": true
        },
        {
          "name": "feeWallet",
          "docs": [
            "CHECK"
          ],
          "writable": true
        }
      ],
      "args": [
        {
          "name": "orderIndex",
          "type": "u64"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "escrowAccount",
      "discriminator": [
        36,
        69,
        48,
        18,
        128,
        225,
        125,
        135
      ]
    },
    {
      "name": "order",
      "discriminator": [
        134,
        173,
        223,
        185,
        77,
        86,
        28,
        51
      ]
    },
    {
      "name": "sellerAccount",
      "discriminator": [
        165,
        133,
        123,
        22,
        51,
        242,
        209,
        52
      ]
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "invalidMode",
      "msg": "Invalid mode"
    },
    {
      "code": 6001,
      "name": "invalidProductType",
      "msg": "Invalid product type"
    },
    {
      "code": 6002,
      "name": "invalidSeller",
      "msg": "Only Seller can trigger"
    },
    {
      "code": 6003,
      "name": "invalidOrderIndex",
      "msg": "Invalid order index"
    },
    {
      "code": 6004,
      "name": "invalidPrice",
      "msg": "Invalid price"
    },
    {
      "code": 6005,
      "name": "invalidModeForDigital",
      "msg": "Digital product must use BTR mode"
    },
    {
      "code": 6006,
      "name": "invalidShippingTime",
      "msg": "Invalid shipping time"
    },
    {
      "code": 6007,
      "name": "digitalNoShipping",
      "msg": "Digital product cannot have shipping time"
    },
    {
      "code": 6008,
      "name": "unauthorized",
      "msg": "unauthorized"
    },
    {
      "code": 6009,
      "name": "invalidBuyer",
      "msg": "Only Buyer can trigger"
    },
    {
      "code": 6010,
      "name": "alreadyFunded",
      "msg": "Order already funded"
    },
    {
      "code": 6011,
      "name": "invalidState",
      "msg": "Invalid State"
    },
    {
      "code": 6012,
      "name": "shippingNotExpired",
      "msg": "Shipping deadline not reached"
    },
    {
      "code": 6013,
      "name": "insufficientEscrow",
      "msg": "Insufficient Escrow"
    },
    {
      "code": 6014,
      "name": "confirmNotExpired",
      "msg": "Confirm deadline not reached"
    },
    {
      "code": 6015,
      "name": "onlyBuyerCanOpenDispute",
      "msg": "Only buyer can open dispute"
    },
    {
      "code": 6016,
      "name": "disputeDeadlineNotReached",
      "msg": "Dispute deadline not reached"
    },
    {
      "code": 6017,
      "name": "discussionNotReached",
      "msg": "Discussion deadline not reached"
    }
  ],
  "types": [
    {
      "name": "escrowAccount",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "order",
            "type": "pubkey"
          },
          {
            "name": "buyer",
            "type": "pubkey"
          },
          {
            "name": "amountLocked",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "order",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "mode",
            "type": "u8"
          },
          {
            "name": "productType",
            "type": "u8"
          },
          {
            "name": "orderName",
            "type": "string"
          },
          {
            "name": "buyerWallet",
            "type": "pubkey"
          },
          {
            "name": "sellerWallet",
            "type": "pubkey"
          },
          {
            "name": "priceLamports",
            "type": "u64"
          },
          {
            "name": "shippingHours",
            "type": "u32"
          },
          {
            "name": "state",
            "type": "u8"
          },
          {
            "name": "orderIndex",
            "type": "u64"
          },
          {
            "name": "bondLamports",
            "type": "u64"
          },
          {
            "name": "feeLamports",
            "type": "u64"
          },
          {
            "name": "totalLamports",
            "type": "u64"
          },
          {
            "name": "sellerFundedAt",
            "type": "i64"
          },
          {
            "name": "markShippedAt",
            "type": "i64"
          },
          {
            "name": "openDisputeAt",
            "type": "i64"
          },
          {
            "name": "sellerRespondAt",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "sellerAccount",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "owner",
            "type": "pubkey"
          },
          {
            "name": "orderCount",
            "type": "u64"
          }
        ]
      }
    }
  ]
};
