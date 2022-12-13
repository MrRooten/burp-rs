class HistoryLog
    @@history = nil
    def self.get_req(index)
        if @@history == nil 
            @@history = RBHttpLog.new
        end

        @@history.get_http_req(index)
    end

   
    def self.get_resp(index)
        if @@history == nil 
            @@history = RBHttpLog.new
        end

        @@history.get_http_resp(index)
    end
end