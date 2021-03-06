#include "parser/rvalue.h"

#include <cassert>

#include "ast/binary_op.h"
#include "ast/real.h"
#include "error.h"
#include "lexer.h"
#include "parser/func.h"
#include "parser/ident.h"
#include "parser/real.h"
#include "parser/do.h"
#include "parser/nested.h"
#include "parser/collection.h"

namespace parser {
ast::Expression* RValue() {
  ast::Expression* mult_expr = nullptr;
  if (lexer::current_token == lexer::TOKEN_ARITH) {
    lexer::Position op_position = lexer::position;
    if (!(lexer::op_str == "-" || lexer::op_str == "+")) {
      Error(op_position, "Unexpected unary ", lexer::op_str);
      return nullptr;
    }
    std::string op = lexer::op_str;
    lexer::NextToken();
    if (op == "-")
      mult_expr = new ast::Real(op_position, -1.0);
  }

  lexer::Position position = lexer::position;
  ast::Expression* rvalue_expr = nullptr;
  switch (lexer::current_token) {
    case lexer::TOKEN_IDENT:
      rvalue_expr = Ident();
      break;

    case lexer::TOKEN_REAL:
    case lexer::TOKEN_ARITH:
      rvalue_expr = Real();
      break;

    case lexer::TOKEN_DO:
      if (mult_expr) {
        Error(position, "Unexpected unary - before do");
        return nullptr;
      }
      rvalue_expr = Do();
      break;

    case '(':
      rvalue_expr = Nested();
      break;

    case '[':
    case '|':
      if (mult_expr) {
        Error(position, "Unexpected unary - before collection");
        return nullptr;
      }
      rvalue_expr = Collection();
      break;

    case lexer::TOKEN_FUNC:
      if (mult_expr) {
        Error(position, "Unexpected unary - before func");
        return nullptr;
      }
      rvalue_expr = Func();
      break;

    default:
      Error(position,
            "Expected identifier or real, got '",
            (char)lexer::current_token,
            "' [",
            lexer::current_token,
            "]");
      return nullptr;
  };

  if (mult_expr)
    return new ast::BinaryOp(position, "*", mult_expr, rvalue_expr);
  return rvalue_expr;
}
}  // end namespace parser
