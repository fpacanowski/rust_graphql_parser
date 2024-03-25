# frozen_string_literal: true

require_relative "rust_graphql_parser/translate"
require_relative "rust_graphql_parser/version"
require_relative "rust_graphql_parser/rust_graphql_parser"
require_relative "another_parser/another_parser"

module RustGraphqlParser
  class Error < StandardError; end
  # Your code goes here...
end
