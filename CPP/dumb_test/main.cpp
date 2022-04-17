#include <iostream>
#include <fstream>
#include <vector>
#include <string>
#include <map>


const char c_NATURAL = 'N';
const char c_ASSIGN  = '=';


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
    Interpreter(std::vector<std::string> &lines)
    {
        text = lines;
        makeTokens();
    }
    void advanceRead(int step=1)
    {
        read_pos += step;
        current_char = line[read_pos];
    }
    std::string removeWhitespace(std::string &input)
    {
        for (unsigned int i = 0; i < input.size(); i++)
        {
            if (input[i] == ' ')
            {
                input.erase(i, i+1);
            }
        }
        return input;
    }
    void makeTokens()
    {
        for (unsigned int i = 0; i < text.size(); i++)
        {
            line = text[i];
            line_length = line.size();
            std::cout << "Current line: " << i+1 << std::endl;
            // std::cout << "Line size: " << line_length << std::endl;
            while (read_pos < line_length)
            {
                // std::cout << (read_pos < line_length) << std::endl;
                // std::cout << current_char << std::endl;
                switch (current_char)
                {
                case c_NATURAL:
                    // std::cout << "Natural!" << std::endl;
                    natural variable;
                    advanceRead(2);
                    while (current_char != ' ')
                    {
                        var_name.push_back(current_char);
                        advanceRead();
                    }
                    // std::cout << "Var name: " << var_name << std::endl;
                    naturals[var_name] = variable;
                    while (current_char != ';')
                    {
                        if (current_char == c_ASSIGN)
                        {
                            isAssigning = true;
                            advanceRead();
                        }
                        if (isAssigning)
                        {
                            var_val.push_back(current_char);
                        }
                        advanceRead();
                    }
                    std::cout << "Var value: " << var_val << std::endl;
                    var_val = removeWhitespace(var_val);
                    // for (unsigned int i = 0; i < var_val.size(); i++)
                    // {
                    //     if (var_val[i] == '+')
                    //     {
                    //         var_val.erase(i, i+1);
                    //     }
                    // }
                    // std::cout << "whitespace erased!" << std::endl;
                    naturals[var_name].value = std::stoi(var_val);
                    std::cout << "Variable: " << var_name << ":" << naturals[var_name].value << std::endl;
                    isAssigning = false;
                    var_name.clear();
                    var_val.clear();
                    break;

                default:
                    // std::cout << "Default!" << std::endl;
                    advanceRead();
                    break;
                }
            }
            // std::cout << (read_pos < line_length) << std::endl;
            read_pos = -1;
        }
    }
};


// int main()
int main(int argc, char *argv[])
{
    std::vector<std::string> lines;
    std::string line;
    std::string filepath = argv[argc - 1];
    std::cout << filepath << std::endl;
    std::ifstream script(filepath);
    // std::ifstream script("build/win/test.mfp");
    while (std::getline(script, line))
    {
        lines.push_back(line);
        std::cout << line << std::endl;
    }
    script.close();
    Interpreter interpreter(lines);
    return 0;
}