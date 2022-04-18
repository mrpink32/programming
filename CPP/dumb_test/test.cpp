#include <iostream>
#include <fstream>
#include <vector>
#include <string>
#include <map>


const char c_SAPCE      = ' ';
const char c_LINEEND    = ';';
const char c_ASSIGN     = '=';
const char c_PLUS       = '+';
const char c_MINUS      = '-';
const char c_MULTI      = '*';
const char c_divition   = '/';
const char c_NATURAL    = 'N';
const char c_ZAHL       = 'Z';


struct natural
{
    unsigned long long int value;
};

struct zahl
{
    signed long long int value;
};


class Interpreter
{
    std::map<std::string, natural> naturals;
    std::map<std::string, zahl> zahls;
    std::vector<std::string> text;
    bool isAssigning = false;
    std::string var_name;
    std::string var_val;
    int read_pos = -1;
    char current_char;
    std::string line;
    int line_length;
public:
    Interpreter(std::vector<std::string>& lines)
    {
        text = lines;
        makeTokens();
    }
    void advanceRead(int step = 1)
    {
        read_pos += step;
        current_char = line[read_pos];
    }
    void /*std::string*/ removeWhitespace(std::string& input)
    {
        for (unsigned int i = 0; i < input.size(); i++)
        {
            if (input[i] == c_SAPCE)
            {
                input.erase(input.begin() + i);
            }
        }
        // return input;
    }
    void makeTokens()
    {
        for (unsigned int i = 0; i < text.size(); i++)
        {
            line = text[i];
            std::cout << "Current line: " << line << std::endl;
            removeWhitespace(line);
            line_length = line.size();
            std::cout << "Current line number: " << i + 1 << std::endl;
            std::cout << "Line size: " << line_length << std::endl;
            std::cout << "Current line: " << line << std::endl;
            while (read_pos < line_length)
            {
                advanceRead();
                switch (current_char)
                {
                case c_NATURAL:
                    natural naturalTemp;
                    advanceRead();
                    while (current_char != c_LINEEND && current_char != c_ASSIGN)
                    {
                        var_name.push_back(current_char);
                        advanceRead();
                    }
                    naturals[var_name] = naturalTemp;
                    std::cout << "Variable name assigned: " << var_name << std::endl;
                    if (current_char == c_ASSIGN)
                    {
                        while (current_char != c_LINEEND)
                        {
                            advanceRead();
                            var_val += current_char;
                        }

                        // todo check if var_val contains an operator
                        // if true handle operation

                        naturals[var_name].value = std::stoi(var_val);
                        std::cout << "Value assigned: " << naturals[var_name].value << std::endl;
                    }
                    var_name.clear();
                    var_val.clear();
                    break;
                case c_ZAHL:
                    zahl zahlTemp;
                    advanceRead();
                    while (current_char != c_LINEEND && current_char != c_ASSIGN)
                    {
                        var_name.push_back(current_char);
                        advanceRead();
                    }
                    zahls[var_name] = zahlTemp;
                    if (current_char == c_ASSIGN)
                    {
                        advanceRead();
                        zahls[var_name].value = current_char;
                    }
                    break;
                default:
                    break;
                }
            }
            read_pos = -1;
        }
    }
};


int main(int argc, char* argv[])
{
    std::vector<std::string> lines;
    std::string line;
    std::string filepath = argv[argc - 1];
    std::cout << filepath << std::endl;
    std::ifstream script(filepath);
    while (std::getline(script, line))
    {
        lines.push_back(line);
        std::cout << line << std::endl;
    }
    script.close();
    Interpreter interpreter(lines);
    return 0;
}