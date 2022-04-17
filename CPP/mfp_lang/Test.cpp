#include <iostream>
#include <fstream>
#include <vector>
#include <string>
#include <map>


const char c_NATURAL = 'N';
const char c_ZAHL = 'Z';
const char c_ASSIGN = '=';
const char c_PLUS = '+';
const char c_MINUS = '-';
const char c_LINEEND = ';';
const char c_SAPCE = ' ';


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
    natural naturalTemp;
    int read_pos = -1;
    char current_char;
    std::string line;
    int line_length;
    zahl zahlTemp;
public:
    Interpreter(std::vector<std::string> lines)
    {
        text = lines;
        makeTokens();
    }
    void advanceRead(int step = 1)
    {
        read_pos += step;
        current_char = line[read_pos];
    }
    std::string removeWhitespace(std::string& input)
    {
        for (unsigned int i = 0; i < input.size(); i++)
        {
            if (input[i] == c_SAPCE)
            {
                input.erase(input.begin() + i);
            }
        }
        return input;
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
                    advanceRead();
                    while (current_char != c_LINEEND && current_char != c_ASSIGN)
                    {
                        var_name.push_back(current_char);
                        advanceRead();
                    }
                    naturals[var_name] = naturalTemp;
                    if (current_char == c_ASSIGN)
                    {
                        advanceRead();
                        naturals[var_name].value = current_char;
                    }
                    break;
                case c_ZAHL:
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