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


echo "Remove old files"

mkdir "old"

touch old/{a..z}.txt
# date in year-month-day format
touch -d '2016-01-01' old/{A..Z}.txt
# remove file older then 4 years, like {A..Z}.txt
rmd --older 4y

for i in old/{a..z}.txt ;  do
    [[ -e "$i" ]] || exit 1
done

for i in old/{A..Z}.txt ; do 
    [[ -e "$i" ]] && exit 1
done


echo "Remove new files"

mkdir new
touch new/{a..z}.txt
# date in year-month-day format
touch -d '2017-01-01' new/{A..Z}.txt

# remove file newer than 2 year in new subdirectory
rmd --newer 2y new

for i in new/{a..z}.txt ;  do
    [[ -e "$i" ]] && exit 1
done

for i in new/{A..Z}.txt ; do 
    [[ -e "$i" ]] || exit 1
done

echo "Remove small files"
mkdir small

for i in small/{a..z}.txt ; do
    dd if=/dev/full of="$i" bs=1 count=$(( RANDOM % 100 + 50 )) &> /dev/null
done

for i in small{A..Z}.txt ; do
    dd if=/dev/full of="$i" bs=1 count=$(( RANDOM % 1000 + 1000 )) &> /dev/null
done

rmd --smaller 1kb small

for i in small/{a..z}.txt ; do
    [[ -e "$i" ]] && exit 1
done

for i in small{A..Z}.txt ; do
    [[ -e "$i" ]] || exit 1
done

echo "Remove large files"
mkdir large

for i in large/{a..z}.txt ; do
    dd if=/dev/full of="$i" bs=1 count=$(( RANDOM % 100 + 50 )) &> /dev/null
done

for i in large/{A..Z}.txt ; do
    dd if=/dev/full of="$i" bs=1 count=$(( RANDOM % 1000 + 1000 )) &> /dev/null
done

rmd --larger 1kb large

for i in large/{a..z}.txt ; do
    [[ -e "$i" ]] || exit 1
done

for i in large/{A..Z}.txt ; do
    [[ -e "$i" ]] && exit 1
done

echo "clean empty directories"
mkdir -p not_clean/{a..z}
touch not_clean/{a..z}/{a..z}.txt

cp -r not_clean clean

rmd -d not_clean
for dir in not_clean/{a..z} ; do
    [[ -e "$dir" ]] || exit 1
done

rmd -d -c clean
(( count = 0 ))
for dir in clean/{a..z} ; do
    if [[ -e "$dir" ]] ; then
        (( count++ ))
    fi
done

(( count == 1 )) || exit 1


mkdir verbose_test
mkdir -p verbose_test/{a..z}

# 26 * 26 files
for dir in verbose_test/* ; do
    for file in {a..z}.txt ; do
        echo "abcd" > "$dir"/"$file"
    done
done

# total size (4 + 1) * 26 * 26 = 2704, there's the newline
rmd -vv -c --smaller 1kb verbose_test | grep '3.38 kb freed' || exit 1

for dir in verbose_test/{a..z} ; do
    [[ -e "$dir" ]] && exit 1
done

for i in {a..f} ; do 
    dd if=/dev/random of=verbose_test/"$i" count=1000 bs=1 &> /dev/null
done

echo -e "y\nn\ny\nn\ny\n" | rmd -vv -i verbose_test/* | grep '3.00 kb freed' || exit 1


echo "Done"
