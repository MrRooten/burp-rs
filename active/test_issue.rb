require 'json'
class RBModule_test_issue
    def initialize
    end
    def metadata
        grades = { "name" => "test_issue", 
            "description" => "" }
        return grades
    end

    def passive_run(index)
        #scan("http://127.0.0.1:8009")
        puts "test2 indexdf: #{index}"
    end

end
