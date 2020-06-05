# Methods list & usage

## 1. Admin method

### 0-1. Install swap contract

```bash
clif contract run wasm swap_install.wasm 0.1 --from elsa
```

### 0-2. Check contract address and memorize

```bash
clif contract query address $(clif keys show elsa -a)
```

```json
{
    "name": "swap_hash",
    "key": {
        "hash": {
        "hash": "fridaycontracthash1n9jsnzahytdxvw2ac996r3kdctmggvaeppnmvs9xl92sa9734lsqjzqs6a"
        }
    }
},
    {
    "name": "swap_proxy",
    "key": {
        "hash": {
        "hash": "fridaycontracthash1ktzdlh77y904num47wdry6qgftvzzfdket6fyvgjtr8uhqv0pnhq0dq4np"
        }
    }
}
```

Write down both of addresses. `swap_hash` is core contract logic, and `swap_proxy` is actual runner that keeps context in contract level.

### 1. Set swap hash

Contract of Execution engine has 2 types. One is logic contract, and another is proxy contract. The context is limited only user-level context if user executes a logic contract. So, the state cannot be share to other users. So, if you want to develop in general perception of 'contract', the context should be global and proxy contract does it.

In this case, for avoiding inputting the address of the logic contract again, as this contract is executed only by admin, just set once and forget it.

```json
[
   {
      "name":"method",
      "value":{
         "cl_type":{
            "simple_type":"STRING"
         },
         "value":{
            "str_value":"set_swap_hash"
         }
      }
   },
   {
      "name":"hash",
      "value":{
         "cl_type":{
            "simple_type":"KEY"
         },
         "value":{
            "key":{
               "hash":{
                  "hash":"<address>"
               }
            }
         }
      }
   }
]
```

Example:

```bash
clif contract run hash fridaycontracthash1ktzdlh77y904num47wdry6qgftvzzfdket6fyvgjtr8uhqv0pnhq0dq4np '[{"name":"method","value":{"cl_type":{"simple_type":"STRING"},"value":{"str_value":"set_swap_hash"}}},{"name":"hash","value":{"cl_type":{"simple_type":"KEY"},"value":{"key":{"hash":{"hash":"fridaycontracthash1n9jsnzahytdxvw2ac996r3kdctmggvaeppnmvs9xl92sa9734lsqjzqs6a"}}}}}]' 0.1 --from elsa
```

### 2. Insert an allowance cap of low level verification in KYC

Hdac has two levels of KYC. If an user get the lower level and if the user has more tokens than the designate amount, the user is prohibited to get swapped tokens. For working well, admin should insert the value. This method works for this.

```json
[
   {
      "name":"method",
      "value":{
         "cl_type":{
            "simple_type":"STRING"
         },
         "value":{
            "str_value":"insert_kyc_allowance_cap"
         }
      }
   },
   {
      "name":"cap",
      "value":{
         "cl_type":{
            "simple_type":"U512"
         },
         "value":{
            "u512":{
               "value":"<value>"
            }
         }
      }
   }
]
```

Example:

```bash
clif contract run hash fridaycontracthash1ktzdlh77y904num47wdry6qgftvzzfdket6fyvgjtr8uhqv0pnhq0dq4np '[{"name":"method","value":{"cl_type":{"simple_type":"STRING"},"value":{"str_value":"insert_kyc_allowance_cap"}}},{"name":"cap","value":{"cl_type":{"simple_type":"U512"},"value":{"u512":{"value":"10000000"}}}}]' 0.1 --from elsa
```

### 3. Insert snapshot record

For recording snapshot information. Admin stores address-amount pair into the contract and the information is used when an user requests toekn swap.

```json
[
   {
      "name":"method",
      "value":{
         "cl_type":{
            "simple_type":"STRING"
         },
         "value":{
            "str_value":"insert_snapshot_record"
         }
      }
   },
   {
      "name":"address",
      "value":{
         "cl_type":{
            "simple_type":"STRING"
         },
         "value":{
            "str_value":"<Ver 1 address>"
         }
      }
   },
   {
      "name":"amount",
      "value":{
         "cl_type":{
            "simple_type":"U512"
         },
         "value":{
            "u512":{
               "value":"<amount>"
            }
         }
      }
   }
]
```

Examples:

```bash
clif contract run hash fridaycontracthash1ktzdlh77y904num47wdry6qgftvzzfdket6fyvgjtr8uhqv0pnhq0dq4np '[{"name":"method","value":{"cl_type":{"simple_type":"STRING"},"value":{"str_value":"insert_snapshot_record"}}},{"name":"address","value":{"cl_type":{"simple_type":"STRING"},"value":{"str_value":"HLkXSESzSaDZgU25CQrmxkjRayKfs5xBFK"}}},{"name":"amount","value":{"cl_type":{"simple_type":"U512"},"value":{"u512":{"value":"20000000"}}}}]' 0.1 --from elsa
```

### 4. Insert KYC data

After the user passes the KYC step, KYC information will be inserted with this contract execution. And, small token for executing swap contract will be transfered to the account of the user.

```json
[
   {
      "name":"method",
      "value":{
         "cl_type":{
            "simple_type":"STRING"
         },
         "value":{
            "str_value":"insert_kyc_data"
         }
      }
   },
   {
      "name":"address",
      "value":{
         "cl_type":{
            "list_type":{
               "inner":{
                  "simple_type":"U8"
               }
            }
         },
         "value":{
            "bytes_value":"<address>"
         }
      }
   },
   {
      "name":"kyc_level",
      "value":{
         "cl_type":{
            "simple_type":"U512"
         },
         "value":{
            "u512":{
               "value":"1" // 1: Lower level - 2: Higher level
            }
         }
      }
   }
]
```

#### NOTE

You should not change the name `address`. It reads that key and changed from bech32 encoded address to base64 encoded string for execution engine.

Example:

```bash
clif contract run hash fridaycontracthash1ktzdlh77y904num47wdry6qgftvzzfdket6fyvgjtr8uhqv0pnhq0dq4np '[{"name":"method","value":{"cl_type":{"simple_type":"STRING"},"value":{"str_value":"insert_kyc_data"}}},{"name":"address","value":{"cl_type":{"list_type":{"inner":{"simple_type":"U8"}}},"value":{"bytes_value":"friday1k568qc388n6x5ks8hkwly2q9ruepns8rr9sgqyjxk9cy6a2qq8gs4v2kpm"}}},{"name":"kyc_level","value":{"cl_type":{"simple_type":"U512"},"value":{"u512":{"value":"1"}}}}]' 0.1 --from elsa
```

### 5. Update KYC data

If the user has been passed the lower level and should be changed into higher level, execute the method. If the user asks for additional token, admin should transfer the token manually with an additional transaction.

```json
[
   {
      "name":"method",
      "value":{
         "cl_type":{
            "simple_type":"STRING"
         },
         "value":{
            "str_value":"update_kyc_level"
         }
      }
   },
   {
      "name":"address",
      "value":{
         "cl_type":{
            "list_type":{
               "inner":{
                  "simple_type":"U8"
               }
            }
         },
         "value":{
            "bytes_value":"<address>"
         }
      }
   },
   {
      "name":"kyc_level",
      "value":{
         "cl_type":{
            "simple_type":"U512"
         },
         "value":{
            "u512":{
               "value":"2" // 1: Lower level - 2: Higher level
            }
         }
      }
   }
]
```

#### NOTE

You should not change the name `address`. It reads that key and changed from bech32 encoded address to base64 encoded string for execution engine.

Example:

```bash
clif contract run hash fridaycontracthash1ktzdlh77y904num47wdry6qgftvzzfdket6fyvgjtr8uhqv0pnhq0dq4np '[{"name":"method","value":{"cl_type":{"simple_type":"STRING"},"value":{"str_value":"update_kyc_level"}}},{"name":"address","value":{"cl_type":{"list_type":{"inner":{"simple_type":"U8"}}},"value":{"bytes_value":"friday1k568qc388n6x5ks8hkwly2q9ruepns8rr9sgqyjxk9cy6a2qq8gs4v2kpm"}}},{"name":"kyc_level","value":{"cl_type":{"simple_type":"U512"},"value":{"u512":{"value":"2"}}}}]' 0.1 --from elsa
```

## 2. User methods

### 1. Get token

Before the swap process, you need to verify both of previous & new mainnet key. New mainnet system automatically verifies the new mainnet key, and the contract verifies the previous mainnet key. The contract contains ECDSA verification logic it can check the meesage, key, and signature. After signature verification, token swap logic will be triggered in the way of offchain.

```json
[
   {
      "name":"method",
      "value":{
         "cl_type":{
            "simple_type":"STRING"
         },
         "value":{
            "str_value":"get_token"
         }
      }
   },
   {
      "name":"my_hash",
      "value":{
         "cl_type":{
            "simple_type":"KEY"
         },
         "value":{
            "key":{
               "hash":{
                  "hash":"<logic_contract_hash_address>"
               }
            }
         }
      }
   },
   {
      "name":"ver1_pubkey",
      "value":{
         "cl_type":{
            "list_type":{
               "inner":{
                  "simple_type":"STRING"
               }
            }
         },
         "value":{
            "list_value":{
               "values":[
                  {
                     "str_value":"<ver1_pubkey>"
                  }
               ]
            }
         }
      }
   },
   {
      "name":"message",
      "value":{
         "cl_type":{
            "list_type":{
               "inner":{
                  "simple_type":"STRING"
               }
            }
         },
         "value":{
            "list_value":{
               "values":[
                  {
                     "str_value":"<hashed_message>"
                  }
               ]
            }
         }
      }
   },
   {
      "name":"signature",
      "value":{
         "cl_type":{
            "list_type":{
               "inner":{
                  "simple_type":"STRING"
               }
            }
         },
         "value":{
            "list_value":{
               "values":[
                  {
                     "str_value":"<signature>"
                  }
               ]
            }
         }
      }
   }
]
```

Example:

```bash
clif contract run hash fridaycontracthash1ktzdlh77y904num47wdry6qgftvzzfdket6fyvgjtr8uhqv0pnhq0dq4np '[{"name":"method","value":{"cl_type":{"simple_type":"STRING"},"value":{"str_value":"get_token"}}},{"name":"my_hash","value":{"cl_type":{"simple_type":"KEY"},"value":{"key":{"hash":{"hash":"fridaycontracthash1n9jsnzahytdxvw2ac996r3kdctmggvaeppnmvs9xl92sa9734lsqjzqs6a"}}}}},{"name":"ver1_pubkey","value":{"cl_type":{"list_type":{"inner":{"simple_type":"STRING"}}},"value":{"list_value":{"values":[{"str_value":"02c4ef70543e18889167ca67c8aa28c1d4c259e89cb34483a8ed6cfd3a03e8246b"}]}}}},{"name":"message","value":{"cl_type":{"list_type":{"inner":{"simple_type":"STRING"}}},"value":{"list_value":{"values":[{"str_value":"69046d44e3d75d48436377626372a44a5066966b5d72c00b67769c1cc6a8619a"}]}}}},{"name":"signature","value":{"cl_type":{"list_type":{"inner":{"simple_type":"STRING"}}},"value":{"list_value":{"values":[{"str_value":"24899366fd3d5dfe6740df1e5f467a53f1a3aaafce26d8df1497a925c55b5c266339a95fe6507bd611b0e3b6e74e3bb7f19eeb1165615e5cebe7f40e5765bc41"}]}}}}]' 0.1 --from elsa
```
