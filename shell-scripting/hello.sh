ARR=()

# Populate new array of size $1
echo "Unsorted: "
echo
for ((i = 0; i < $1; i++)); do
    ARR+=($[$RANDOM % 1000])
    echo -n "${ARR[$i]} " 
done
echo
echo

# Bubble Sort
SWAPPED=true
while "$SWAPPED"; do 
    SWAPPED=false
    for ((i = 0; i < $[$1 - 1]; i++)); do
        if (( ARR[i] > ARR[i+1] )); then
            TEMP=${ARR[i]}
            ARR[i]=${ARR[i+1]}
            ARR[i+1]=$TEMP
            SWAPPED=true
        fi
    done
done

echo "Sorted: "
echo
# Print array
for ((i = 0; i < $1; i++)); do
    echo -n "${ARR[$i]} " 
done
echo