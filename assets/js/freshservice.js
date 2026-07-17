let polling = false;

helpdeskInfo();
setInterval(async () => {
    if (!polling) {
        await helpdeskInfo();
    }
}, 15000);

async function helpdeskInfo() {
    polling = true;

    try {
        const response = await fetch(
            "http://localhost:3000/api/integrations/fresh-service",
            {
                method: "GET",
            },
        );

        if (response.ok) {
            const data = await response.json();

            for (const d of data) {
                if (d.ut) {
                    document.getElementById(
                        "unassigned-ticket-value",
                    ).innerHTML = d.ut.total;

                    if (d.ut.total > 0) {
                        await unassignedAlarmTrigger();
                    }
                }
            }
        } else {
            console.error(
                "Failed to fetch helpdesk information. Check the rust console for any errors.",
            );
        }
    } finally {
        polling = false;
    }
}

async function unassignedAlarmTrigger() {
    const response = await fetch(`/api/misc/fs/notify`, {
        method: "GET",
    });

    if (!response.ok) {
        console.error("Failed to play fs notify audio.");
    }
}

const form = document.getElementById("form");
form.addEventListener('submit', async (e) => {
  e.preventDefault();

  const formData = new FormData(form);

  const response = await fetch('/api/freshservice/custom-notify-upload', {
    method: "POST",
    body: formData
  });

  const data = await response.json();
  console.log(data);
});
