class Similary 
    @@matcher = RBSimilary.new
    def self.match(s1, s2)
        @@matcher.sim_calculate(s1,s2)
    end
end