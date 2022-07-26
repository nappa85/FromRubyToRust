require "benchmark/ips"

namespace :benchmark do
    desc 'Benchmarks'
    task benchmark: :environment do
        Benchmark.ips do |benchmark|
            benchmark.report('ruby') do
                MyModel.first.cached_method
            end
            benchmark.compare!
        end
    end
end
