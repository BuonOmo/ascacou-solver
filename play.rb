# frozen_string_literal: true

# TODO: replace this with a correct interface, directly in rust.

$prev_ia, $curr_ia = ARGV

def game_info(fen)
  terminal, score = `./target/release/game-info #{fen.inspect}`.split
  [terminal == "true", score.to_i]
end

def next_position(fen, move)
  `./target/release/game-info #{fen.inspect} #{move.inspect}`.chomp
end

def run_game(ias: ARGV, moves_already_played: 0, fen: nil)
  i = [0, 1].sample
  ia_order = ias.first(2)

  fen ||= `./target/release/game-info`.chomp
  # puts fen
  moves_already_played.times do
    move = `./random-ia #{fen.inspect}`.chomp
    fen = next_position(fen, move)
  end
  puts fen

  score = nil

  loop do
    terminal, score = game_info(fen)
    break if terminal

    i = 1 - i
    ia = ia_order[i]
    move = `#{ia} -d #{1} #{fen.inspect}`.chomp
    # puts "Playing #{move}"
    fen = next_position(fen, move)
  end

  if score < 0
    i = 1 - i
    score = - score
  end
  if score == 0
    return "draw"
  else
    return "winner #{ia_order[i]} score=#{score} tiles=#{fen.split.last}"
  end
end

r = []

40.times do
  result = run_game(moves_already_played: 0, fen: '2bbw/bww1w/w1w1w/1w1bw/wbb1b 2458abdf')
  puts result
  r << result
rescue Interrupt
  break
end

pp r.map{_1.split[3]||"draw"}.tally
