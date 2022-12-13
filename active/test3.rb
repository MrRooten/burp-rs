

class RBModule
    def initialize
    end
    def metadata
        grades = { "name" => "test3", 
            "description" => "" }
        return grades
    end

    def passive_run(index)
        #scan("http://127.0.0.1:8009")
        puts "test3 index: #{index}"
        puts $LOADED_FEATURES
    end

end