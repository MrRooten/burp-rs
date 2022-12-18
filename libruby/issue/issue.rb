class IssueReporter
    @@reporter = RBIssue.new
    def self.add_issue(issue)
        @@reporter.push_issue(issue)
    end
end