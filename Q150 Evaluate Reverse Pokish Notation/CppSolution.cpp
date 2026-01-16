#include <vector>
#include <string>
#include <stack>

using namespace std;

class Solution {
public:
    int evalRPN(vector<string>& tokens) {
        stack<int> st;

        for (const string& t : tokens) {
            if (t == "+" || t == "-" || t == "*" || t == "/") {
                int b = st.top(); st.pop();
                int a = st.top(); st.pop();

                if (t == "+") {
                    st.push(a + b);
                } else if (t == "-") {
                    st.push(a - b);
                } else if (t == "*") {
                    st.push(a * b);
                } else { // "/"
                    // Integer division in C++ truncates toward zero
                    st.push(a / b);
                }
            } else {
                st.push(stoi(t));
            }
        }

        return st.top();
    }
};
