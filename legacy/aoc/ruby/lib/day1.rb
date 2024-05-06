# frozen_string_literal: true

module Day1
  Input   = Struct.new(:input)
  EXAMPLE = Input.new(File.read('examples/example1.txt'))
  INPUT   = Input.new(File.read('inputs/input1.txt'))

  def self.problem_a(input)
    input.input.split.map(&:to_i).each_cons(2).count { |i, j| i < j }
  end

  def self.problem_b(input)
    input.input.split.map(&:to_i).each_cons(3).map(&:sum).each_cons(2).count { |i, j| i < j }
  end
end

__END__

String#split  :: String -> [String]
[A]#map       :: [A] -> (A -> B) -> [B]
[A]#each_pair :: [A] -> [(A, A)]
[A]#count     :: [A] -> (A -> Bool) -> Int

module Day1
  Input = Input(String)

  problem_a(input) = input.split.map{.to_i}.each_pair.count{_1<_2}
end

main = File.read('input.txt') |> Day1.solve
