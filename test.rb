require 'json'
class RBModule
    def metadata
        grades = { "Jane Doe" => 10, "Jim Doe" => 6 }
        return grades
    end
end

print([1,2,3].to_json)
