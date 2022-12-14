require 'json'
class RBModule
    def initialize
    end
    def metadata
        grades = { "name" => "test2", 
            "description" => "" }
        return grades
    end

    def passive_run(index)
        #scan("http://127.0.0.1:8009")
        puts "test2 indexdf: #{index}"
    end

end
