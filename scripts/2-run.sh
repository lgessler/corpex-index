#!/bin/sh
screen -d -m -S shard0 corpex-index/target/release/corpex-index run sharded-sets/set0.fst -p 6112
screen -d -m -S shard1 corpex-index/target/release/corpex-index run sharded-sets/set1.fst -p 6113
screen -d -m -S shard2 corpex-index/target/release/corpex-index run sharded-sets/set2.fst -p 6114
screen -d -m -S shard3 corpex-index/target/release/corpex-index run sharded-sets/set3.fst -p 6115
screen -d -m -S shard4 corpex-index/target/release/corpex-index run sharded-sets/set4.fst -p 6116
screen -d -m -S shard5 corpex-index/target/release/corpex-index run sharded-sets/set5.fst -p 6117
screen -d -m -S shard6 corpex-index/target/release/corpex-index run sharded-sets/set6.fst -p 6118
screen -d -m -S shard7 corpex-index/target/release/corpex-index run sharded-sets/set7.fst -p 6119
