#include <iostream>
#include <fstream>
#include <vector>
#include <sstream>
#include <algorithm>

using namespace std;

pair<bool, int> isArraySafe(const vector<int>& array) {
    int firstNonSafe = 0;
    bool unsafe = false;
    bool increasing = false;

    if (array[1] == array[0]) {
        unsafe = true;
        firstNonSafe = 1;
    } else if (array[1] > array[0]) {
        increasing = true;
    } else {
        increasing = false;
    }

    if (abs(array[1] - array[0]) > 3) {
        firstNonSafe = 1;
        unsafe = true;
    }

    if (!unsafe) {
        for (size_t i = 1; i < array.size(); ++i) {
            if (array[i] == array[i - 1]) {
                firstNonSafe = i;
                unsafe = true;
                break;
            } else if (array[i] > array[i - 1] && !increasing) {
                firstNonSafe = i;
                unsafe = true;
                break;
            } else if (array[i] < array[i - 1] && increasing) {
                firstNonSafe = i;
                unsafe = true;
                break;
            }

            if (abs(array[i] - array[i - 1]) > 3) {
                firstNonSafe = i;
                unsafe = true;
                break;
            }
        }
    }

    return { !unsafe, firstNonSafe };
}

int main(int argc, char* argv[]) {
    if (argc < 2) {
        std::cerr << "Usage: " << argv[0] << " <problemDampener: 0 or 1>" << std::endl;
        return 1;
    }
    bool problemDampener = (std::string(argv[1]) == "1");

    ifstream inputFile("input2.txt");
    if (!inputFile) {
        cerr << "Error opening file!" << endl;
        return 1;
    }

    string line;
    int safeReports = 0;

    while (getline(inputFile, line)) {
        vector<int> array;
        istringstream iss(line);
        int num;
        while (iss >> num) {
            array.push_back(num);
        }

        auto [isSafe, firstNonSafe] = isArraySafe(array);

        if (problemDampener && !isSafe) {
            bool isSafe0 = false, isSafe1 = false;

            if (firstNonSafe > 0) {
                vector<int> array0 = array;
                array0.erase(array0.begin() + firstNonSafe - 1);
                isSafe0 = isArraySafe(array0).first;
            }

            if (firstNonSafe < array.size()) {
                vector<int> array1 = array;
                array1.erase(array1.begin() + firstNonSafe);
                isSafe1 = isArraySafe(array1).first;
            }

            isSafe = isSafe0 || isSafe1;
        }

        safeReports += isSafe ? 1 : 0;
    }

    inputFile.close();

    cout << safeReports << endl;

    return 0;
}
