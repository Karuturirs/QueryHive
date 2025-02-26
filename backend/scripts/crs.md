Vertex AI Search for commerce Service Level Agreement (SLA)

During the Term of the agreement under which Google has agreed to provide Google Cloud Platform to Customer (as applicable, the "Agreement"), the Covered Service will provide a Monthly Uptime Percentage to Customer (the "Service Level Objective" or "SLO") as follows:

Covered Service
	

Monthly Uptime Percentage

Vertex AI Search for commerce
	

99.9%

If Google does not meet the SLO, and if Customer meets its obligations under this SLA, Customer will be eligible to receive the Financial Credits described below. Monthly Uptime Percentage and Financial Credit are determined on a calendar month basis per Project per Region. This SLA states Customer's sole and exclusive remedy for any failure by Google to meet the SLO. Capitalized terms used in this SLA, but not defined in this SLA, have the meaning set forth in the Agreement. If the Agreement authorizes the resale or supply of Google Cloud Platform under a Google Cloud partner or reseller program, then all references to Customer in this SLA mean Partner or Reseller (as applicable), and any Financial Credit(s) will only apply for impacted Partner or Reseller order(s) under the Agreement.
Definitions

The following definitions apply to the SLA:

    "Covered Service" means Vertex AI Search for commerce.
    "Downtime" means more than a 10% percent Error Rate and is measured based on server side Error Rate.
    "Downtime Period" means a period of ten or more consecutive minutes of Downtime. Partial minutes or intermittent Downtime for a period of less than ten minutes will not count towards any Downtime Periods.
    "Error Rate" means the number of Valid Requests that result in a response with HTTP Status 5XX and Code "Internal Error" divided by the total number of Valid Requests during that period, subject to a minimum of 2000 Valid Requests in the measurement period.
    "Financial Credit" means the following:

Monthly Uptime Percentage
	

Percentage of monthly bill for the respective Covered Service in the Region that does not meet SLO that will be credited to Customer's future monthly bills

99.0% - < 99.9%
	

10%

95.0% - < 99.0%
	

25%

< 95.0%
	

50%

    "Monthly Uptime Percentage" means total number of minutes in a month, minus the number of minutes of Downtime suffered from all Downtime Periods in a month, divided by the total number of minutes in a month.
    "Region" means the applicable region identified at https://cloud.google.com/about/locations.
    "Valid Requests" are requests that conform to the Documentation, and that would normally result in a non-error response.

Customer Must Request Financial Credit

In order to receive any of the Financial Credits described above, Customer must notify Google technical support within 30 days from the time Customer becomes eligible to receive a Financial Credit. Customer must also provide Google with log files showing Downtime Periods and the date and time they occurred. If Customer does not comply with these requirements, Customer will forfeit its right to receive a Financial Credit.
Maximum Financial Credit

The maximum aggregate number of Financial Credits issued by Google to Customer for all Downtime Periods in a single billing month will not exceed 50% of the amount due from Customer for the Covered Service in the Region that did not meet SLO for the applicable month. Financial Credits will be in the form of a monetary credit applied to future use of the Covered Service and will be applied within 60 days after the Financial Credit was requested.
SLA Exclusions

The SLA does not apply to any (a) features or services designated pre-general availability (unless otherwise set forth in the associated Documentation); (b) features or services excluded from the SLA (in the associated Documentation); or (c) errors (i) caused by factors outside of Google's reasonable control; (ii) that resulted from Customer's software or hardware or third party software or hardware, or both; (iii) that resulted from abuses or other behaviors that violate the Agreement; or (iv) that resulted from quotas applied by the system or listed in the Documentation or Admin Console.