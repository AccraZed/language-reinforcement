/**=============================================================================
 * @file checksum.cpp
 * @author Amelia Castilla
 * @brief 
 *      HW02 - Calculating the 8, 16, or 32 bit checksum for a given input file
 * 
 *      To Compile: g++ -o checksum checksum.cpp
 * 
 *      To Execute: ./checksum.out textfile.txt checksum_size
 * 
 *      where the files in the command line are in the current directory. 
 *      The text file contains text is mixed case with spaces, punctuation, 
 *      and is terminated by the hexadecimal ‘0A’, inclusive.
 *      (The 0x’0A’ is included in the checksum calculation.)
 *      The checksum_size contains digit(s) expressing the checksum size 
 *      of either 8, 16, or 32 bits 
 * 
 * Class: CIS3360 - Security in Computing – Spring 2021
 * Instructor: Dr. McAlpin
 * Due Date: 18 April 2021
 =============================================================================*/

#include <iostream>
#include <fstream>
#include <sstream>

int main(int argc, char *argv[])
{
    int checksum_size = 0;
    int character_count = 0;
    int checksum = 0;
    
    // Check for valid number of args
    if (argc < 3)
    {
        std::cout << "Invalid syntax." << std::endl;
    }

    // Check for valid checksum size input
    switch (checksum_size = std::atoi(argv[2]))
    {
        case 8:
        case 16:
        case 32:
            break;
        default:
            fprintf(stderr, "Valid checksum sizes are 8, 16, or 32\n");
            std::exit(1);
    }

    // Pull file into a string
    std::ifstream file_input(argv[1]);
    std::stringstream string_stream;
    string_stream << file_input.rdbuf();
    std::string input = string_stream.str();


    // Add all values to the checksum
    int i, j;
    int current_check;
    int str_size = input.size();
    for (i = 0; input[i] != '\n'; i++)
    {
        current_check = 0;

        // If the checksum size is over 8, then multiple numbers need to be bitshifted
        for (j = 0; j < checksum_size; j += 8, i++)
        {
            // Pad ending if need be
            if (i != str_size)
                current_check += input[i];
            else
                current_check += 'X';

            current_check = current_check << 8;    
        }

        checksum += current_check;
    }

    // Truncate the checksum to the appropriate number of bits
    checksum = checksum & ((1 << checksum_size) - 1);


    // Print input
    i = 0;
    while (i < str_size)
    {
        printf(%.80s, input + i);
        i += 80;
    }
    // Output result
    printf("%2d bit checksum is %8lx for all %4d chars\n", checksum_size, checksum, character_count);

    return 0;
}

/**=============================================================================
* I, Amelia Castilla (em745353), affirm that this program is 
* entirely my own work and that I have neither developed my code together with
* any another person, nor copied any code from any other person, nor permitted
* my code to be copied or otherwise used by any other person, nor have I 
* copied, modified, or otherwise used programs created by others. I acknowledge
* that any violation of the above terms will be treated as academic dishonesty.
+=============================================================================*/