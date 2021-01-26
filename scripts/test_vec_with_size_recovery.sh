# # Recovery test example
# create & state # size 0, elems: [], state: consistent
# put e0
# put e1
# put e2 (crashed)
# state (no recovery) # size 2, elems: [e0, e1, e2], state: inconsistent
# state (recovery) # size 2, elems: [e0, e1], state: consistent
# put e2
# delete 2 (crashed)
# state (no recovery) # size 2, elems: [e0, e1, e2], state: inconsistent
# state (recovery) # size 3, elems: [e0, e1, e2], state: consistent

echo "<create & print state>" > ./scripts/test_vec_with_size_recovery_output.txt
cargo run vec_with_size vec_with_size.pool state >> ./scripts/test_vec_with_size_recovery_output.txt
echo -e "\n" >> ./scripts/test_vec_with_size_recovery_output.txt

echo "<put e0>" >> ./scripts/test_vec_with_size_recovery_output.txt
cargo run vec_with_size vec_with_size.pool put e0 >> ./scripts/test_vec_with_size_recovery_output.txt
echo -e "\n" >> ./scripts/test_vec_with_size_recovery_output.txt

echo "<put e1>" >> ./scripts/test_vec_with_size_recovery_output.txt
cargo run vec_with_size vec_with_size.pool put e1 >> ./scripts/test_vec_with_size_recovery_output.txt
echo -e "\n" >> ./scripts/test_vec_with_size_recovery_output.txt

echo "<put e2 (crashed)>" >> ./scripts/test_vec_with_size_recovery_output.txt
crash_put=1 cargo run vec_with_size vec_with_size.pool put e2 >> ./scripts/test_vec_with_size_recovery_output.txt
echo -e "\n" >> ./scripts/test_vec_with_size_recovery_output.txt

echo "<print state (no recovery)>" >> ./scripts/test_vec_with_size_recovery_output.txt
no_recover=1 cargo run vec_with_size vec_with_size.pool state >> ./scripts/test_vec_with_size_recovery_output.txt
echo -e "\n" >> ./scripts/test_vec_with_size_recovery_output.txt

echo "<print state (recovery)>" >> ./scripts/test_vec_with_size_recovery_output.txt
cargo run vec_with_size vec_with_size.pool state >> ./scripts/test_vec_with_size_recovery_output.txt 
echo -e "\n" >> ./scripts/test_vec_with_size_recovery_output.txt

echo "<put e2>" >> ./scripts/test_vec_with_size_recovery_output.txt
cargo run vec_with_size vec_with_size.pool put e2 >> ./scripts/test_vec_with_size_recovery_output.txt
echo -e "\n" >> ./scripts/test_vec_with_size_recovery_output.txt

echo "<delete 2 (crashed)>" >> ./scripts/test_vec_with_size_recovery_output.txt
crash_del=1 cargo run vec_with_size vec_with_size.pool del 2 >> ./scripts/test_vec_with_size_recovery_output.txt
echo -e "\n" >> ./scripts/test_vec_with_size_recovery_output.txt

echo "<print state (no recovery)>" >> ./scripts/test_vec_with_size_recovery_output.txt
no_recover=1 cargo run vec_with_size vec_with_size.pool state >> ./scripts/test_vec_with_size_recovery_output.txt
echo -e "\n" >> ./scripts/test_vec_with_size_recovery_output.txt

echo "<print state (recovery)>" >> ./scripts/test_vec_with_size_recovery_output.txt
cargo run vec_with_size vec_with_size.pool state >> ./scripts/test_vec_with_size_recovery_output.txt
echo -e "\n" >> ./scripts/test_vec_with_size_recovery_output.txt