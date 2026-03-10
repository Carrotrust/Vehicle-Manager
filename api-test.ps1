# this will not work 
Invoke-RestMethod -Uri http://localhost:6570/vehicle

# this should work- as its targetting the correct part
Invoke-RestMethod -Uri http://localhost:6570/vehicle -Method Get 

Invoke-RestMethod -Uri http://localhost:6570/vehicle -Method Post

$Params = @{
    Uri  = "http://localhost:6570/vehicle"
    Method = "Post"
    Body = @{
        manufacturer = 'Tesla'
        model        = 'CyberTruck'
        year         = 2025
    } | ConvertTo-Json
    ContentType = "application/json"
}
Invoke-RestMethod @Params



