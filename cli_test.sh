#! /bin/bash

cargo install --path . --force

if [[ "$CARGO_HOME" ]] ; then
    TMP_PATH="$CARGO_HOME/bin"
else
    TMP_PATH="$HOME/.cargo/bin"
fi

export PATH="$PATH:$TMP_PATH"

cd $(mktemp -d)

echo "Remove some files"
touch {a..z}.txt
rmd {a..f}.txt

for i in  {g..z}.txt ; do
    [[ -e "$i" ]] || exit 1
done


echo "Remove duplicates"
for i in {a..z} ; do
    echo "TEST" > "$i"
done

rmd -d 
COUNT=0
for i in {a..z} ; do
    [[ -e "$i" ]] && (( COUNT++ ))
done

[[ "$COUNT" == '1' ]] || exit 1

echo "Remove duplicates in a different directory"

mkdir -p "SOME/PATH"

echo "test" > "SOME/FILE"
echo "test" > "SOME/PATH/FILE_2"
echo "test" > "SOME/FILE_3"
echo "test" > "DUP_A"
echo "test" > "DUP_B"

rmd -d 'SOME'

COUNT=0
[[ -e "SOME/FILE" ]] && (( COUNT++ ))
[[ -e "SOME/PATH/FILE_2" ]] && (( COUNT++ ))
[[ -e "SOME/FILE_3" ]] && (( COUNT++ ))
[[ -e "DUP_A" ]] || exit 
[[ -e "DUP_B" ]] || exit 
[[ "$COUNT" == '1' ]] || exit 1

echo "Remove Directory"
rmd -rf 'SOME'
[[ -e "SOME" ]] && exit 1


echo "Test Interactive"
touch {a..z}
yes | rmd -i {a..z}

for i in {a..z} ; do
    [[ -e "$i" ]] && exit 1
done

touch {a..z}
yes n | rmd -i {a..z}

for i in {a..z} ; do
    [[ -e "$i" ]] || exit 1
done


echo "Done"
