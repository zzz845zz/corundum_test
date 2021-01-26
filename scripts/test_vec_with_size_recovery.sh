# create & state
# put e0
# put e1
# put e2 (crashed)
# state (no recovery)
# state (recovery)
# put e2
# delete 2 (crashed)
# state (no recovery)
# state (recovery)

echo "<create & print state>" > output.txt
cargo run vec_with_size vec_with_size.pool state >> output.txt
echo -e "\n" >> output.txt

echo "<put e0>" >> output.txt
cargo run vec_with_size vec_with_size.pool put e0 >> output.txt
echo -e "\n" >> output.txt

echo "<put e1>" >> output.txt
cargo run vec_with_size vec_with_size.pool put e1 >> output.txt
echo -e "\n" >> output.txt

echo "<put e2 (crashed)>" >> output.txt
crash_put=1 cargo run vec_with_size vec_with_size.pool put e2 >> output.txt
echo -e "\n" >> output.txt

echo "<print state (no recovery)>" >> output.txt
no_recover=1 cargo run vec_with_size vec_with_size.pool state >> output.txt
echo -e "\n" >> output.txt

echo "<print state (recovery)>" >> output.txt
cargo run vec_with_size vec_with_size.pool state >> output.txt 
echo -e "\n" >> output.txt

echo "<put e2>" >> output.txt
cargo run vec_with_size vec_with_size.pool put e2 >> output.txt
echo -e "\n" >> output.txt

echo "<delete 2 (crashed)>" >> output.txt
crash_del=1 cargo run vec_with_size vec_with_size.pool del 2 >> output.txt
echo -e "\n" >> output.txt

echo "<print state (no recovery)>" >> output.txt
no_recover=1 cargo run vec_with_size vec_with_size.pool state >> output.txt
echo -e "\n" >> output.txt

echo "<print state (recovery)>" >> output.txt
cargo run vec_with_size vec_with_size.pool state >> output.txt
echo -e "\n" >> output.txt