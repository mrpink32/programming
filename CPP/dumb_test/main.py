import sys

TT_INT      = 'INT'
TT_FLOAT    = 'FLOAT'
TT_PLUS     = 'PLUS'
TT_MINUS    = 'MINUS'
TT_MULTIPLY = 'MUL'
TT_DIVIDE   = 'DIV'
TT_LPAREN   = 'LPAREN'
TT_RPAREN   = 'RPAREN'
NUMBERS     = '0123456789'

class Token:
    def __init__(self, _type, value=None):
        self.type = _type
        self.value = value
    def __repr__(self):
        if self.value: return f'{self.type}:{self.value}'
        return f'{self.type}'


class Lexer:
    def __init__(self, line):
        self.line = line
        self.read_pos = 0
        self.make_tokens()
    def advance_read(self):
        self.current_char = self.line[self.read_pos] if self.read_pos < len(self.line) else None
        self.read_pos += 1
    def make_tokens(self):
        tokens = []
        while self.current_char != None:
            match self.current_char:
                case '\t':
                    self.advance_read()
                case '+':
                    tokens.append(Token(TT_PLUS))
        tokens = []
        current_keyword = ""
        for char in self.line:
            self.advance_read()
            current_keyword += self.current_char
            if self.current_char == ' ':
                print(current_keyword)
            


        return tokens


def main(file_path):
    # lines = []
    # with open(file_path, "r") as script:
    #     for line in script:
    #         lines.append(line)
    # Lexer(lines)
    line = None
    with open(file_path, "r") as script:
        line = script.readline()
    Lexer(line)
        

if __name__ == "__main__":
    main(sys.argv[len(sys.argv) - 1])
