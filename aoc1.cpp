#include <iostream>
#include <fstream>
#include <vector>
#include <sstream>
#include <algorithm>
#include <numeric>

using namespace std;

int main() {
    ifstream inputFile("input1.txt");
    if (!inputFile) {
        cerr << "Error opening file!" << std::endl;
        return 1;
    }

    vector<uint> list1, list2;
    string line;

    while (getline(inputFile, line)) {
        istringstream iss(line);
        uint val1, val2;
        iss >> val1 >> val2;
        list1.push_back(val1);
        list2.push_back(val2);
    }

    inputFile.close();

    sort(list1.begin(), list1.end());
    sort(list2.begin(), list2.end());

    vector<uint> distanceVec(list1.size());
    for (uint i = 0; i < list1.size(); ++i) {
        distanceVec[i] = list1[i] - list2[i];
    }

    uint totalDistance = accumulate(distanceVec.begin(), distanceVec.end(), 0, [](int sum, int val) { return sum + abs(val); });

    cout << "Skibidi Distance: " << totalDistance << std::endl;

    vector<uint> countScores(list1.size(), 0);
    bool found = false;

    for (uint i = 0; i < list1.size(); ++i) {
        for (uint j = 0; j < list2.size(); ++j) {
            if (list1[i] == list2[j]) {
                found = true;
                countScores[i]++;
            } else if (found) {
                found = false;
                break;
            }
        }
    }

    int similarityScoresSum = 0;
    for (uint i = 0; i < list1.size(); ++i) {
        similarityScoresSum += countScores[i] * list1[i];
    }

    cout << "Ohio Distance: " << similarityScoresSum << endl;

    return 0;
}
