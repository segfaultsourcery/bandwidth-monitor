# Acquire credentials

Start by creating a service account at the Google Developer Console:

1. Open https://console.developers.google.com/apis/library


![Console homepage](images/console-homepage.png)


2. Enable it and create a project:


![Enable sheets](images/enable-sheets.png)


3. At Credentials, create a service account:

![Create service account](images/crate-service-account.png)


4. Keep defaults at the next form and click Create. Save the resulting private key file in a directory where your application can read it:

![Fill out form](images/fill-out-form.png)

5. Next, click on “Manage Service accounts”:

![Manage service sccounts](images/manage-sa.png)


6. Copy the email address of your service account:

![Copy service account email](images/copy-email.png)

7. Grant permission to the service account to the spreadsheet:

![Share with service account](images/share-with-sa.png)

8. Store the private key in `secret.json` (you also can choose another name and specify the path)