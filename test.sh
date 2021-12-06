#! /usr/bin/env sh

for file in tests/*.len; do
	./target/debug/lengthlvm $file
	if test $? -ne 0; then
		echo "An error occurred while compiling " 
		exit 1
	fi
	object="tests/$(basename $file .len).o"
	mv out.o "$object"
	mv out.ir "tests/$(basename $file .len).ir"
	gcc $object -llength -Lliblength/builddir -o "tests/$(basename $file .len)"
	"tests/$(basename $file .len)" > "tests/$(basename $file .len).actual"
	diff "tests/$(basename $file .len).actual" "tests/$(basename $file .len).expected" > /dev/null
	if test $? -ne 0
	then
		echo "An error occurred while testing $file" 
		fail=yes
	fi
done

if [ -n "$fail" ]; then
	echo "1 or more tests failed"
	exit 1
else
	echo "All tests passed!"
fi