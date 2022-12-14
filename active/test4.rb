

class RBModule
    def initialize
    end
    def metadata
        grades = { "name" => "test4", 
            "description" => "" }
        return grades
    end

    def passive_run(index)
        #scan("http://127.0.0.1:8009")
        puts "test4 index: 123 #{index}"
    end

end