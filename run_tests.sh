echo "PROTOTYPE TESTS:"

./tests/prototype/runall.sh ./ash.py ./libask.so

echo "BASELINE TESTS:"

./tests/final/baseline/runall.sh ./ash.py ./libask.so

echo "PROTOTYPE TEST RESULTS:"

./tests/diffscore.py ./tests/prototype/actual/proto_test1.log ./tests/prototype/expected/proto_test1.log 
./tests/diffscore.py ./tests/prototype/actual/proto_test2.log ./tests/prototype/expected/proto_test2.log 
./tests/diffscore.py ./tests/prototype/actual/proto_test3.log ./tests/prototype/expected/proto_test3.log 
./tests/diffscore.py ./tests/prototype/actual/proto_test4.log ./tests/prototype/expected/proto_test4.log 
./tests/diffscore.py ./tests/prototype/actual/proto_test5.log ./tests/prototype/expected/proto_test5.log 
./tests/diffscore.py ./tests/prototype/actual/proto_test6.log ./tests/prototype/expected/proto_test6.log 
./tests/diffscore.py ./tests/prototype/actual/proto_test7.log ./tests/prototype/expected/proto_test7.log 
echo
echo
echo "BASELINE TEST RESULTS:"

./tests/diffscore.py ./tests/final/baseline/expected/branch.log ./tests/final/baseline/actual/branch.log
./tests/diffscore.py ./tests/final/baseline/expected/cmp.log ./tests/final/baseline/actual/cmp.log
./tests/diffscore.py ./tests/final/baseline/expected/locals.log ./tests/final/baseline/actual/locals.log
./tests/diffscore.py ./tests/final/baseline/expected/pointers.log ./tests/final/baseline/actual/pointers.log
