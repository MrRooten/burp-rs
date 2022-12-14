class IssueReporter
    @@reporter = nil
    def self.add_issue(issue)
        if @@reporter = nil 
            @@reporter = RBIssue.new
        end

        @@reporter.push_issue(issue)
    end
end