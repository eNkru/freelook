const { ipcRenderer } = require('electron');


ipcRenderer.on('registerCalloutObserver', function (event, data) {
    setTimeout(() => {

        console.log("Trying to register callout observer");
        const layerDiv = document.querySelector(".ms-Layer-content");
        if (layerDiv) {
            const observer = new MutationObserver((mutations) => {

                for (const mutation of mutations) {
                    console.log(mutation);
                }

                const emailDiv = document.querySelector("._9Ie6sVh0VX-ro9NAPwFKb");
                // First childnode == email adress, Second childnode == subject, Third child node == email body
                if (emailDiv) {
                    ipcRenderer.send('showNotification', { address: emailDiv.childNodes[0].textContent, subject: emailDiv.childNodes[1].textContent });
                }
            });

            console.log("Starting observer for notifications");
            observer.observe(layerDiv, { childList: true, subtree: true });

        } else {
            console.log("Could not register notification observer. Notifications not supported");
            alert("Could not register notification observer. Notifications not supported");
        }
    }, 5000);
});
