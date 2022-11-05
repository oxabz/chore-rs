let participants = [];
let res = null;

let urls = (data)=>{
    let name = encodeURIComponent(data.name);
    let start = data.start;
    let end = data.end;
    let participants = data.participants;
    let period = data.period;
    
    let origin = window.location.host;

    start = Date.parse(start);
    end = Date.parse(end);
    start = Math.floor(start/8.64e7);
    end = Math.floor(end/8.64e7);

    return participants.map((participant,i)=>{
        return `webcal://${origin}/${period}/${name}?start=${start}&end=${end}&participent=${participant}&index=${i}&count=${participants.length}`
    })
}

let scrollTo = (selector) => {
    let element = $(selector);
    window.scrollTo(0, element.offset().top, "smooth");
};

let displayResults = (res) => {
    let $results = $("#results");
    $results.empty();
    res.forEach((result, i) => {
        $results.append(`<li>
            ${participants[i]} :
            <a class="link link-success" href="${result}">${result}</a>
        </li>`);
    });
    $("#done").show();
}

// display participents 
let participantRows = $("#participant-rows");
let displayParticipents = () => {
    participantRows.empty();
    if (participants.length === 0) {
        participantRows.append("<tr><td class='text-base-content/50 text-center' colspan='2'>No participents yet</td></tr>");
    }
    else {
        participants.forEach((participent, index) => {
            participantRows.append(`
            <tr>
                <th>${index + 1}</th>
                <td class="flex justify-between">
                    <p class="text-base-content">${participent}</p>
                    <button type="button" class="btn btn-error btn-sm" onclick="removeParticipant('${participent}')">Remove</button>
                </td>
            </tr>
            `);
        });
    }
};

let removeParticipant = (participant) => {
    participants = participants.filter((p) => p !== participant);
    displayParticipents();
}

let copyToClipboard = () => {
    if (res === null) {
        return;
    }
    let rstr = res.map((r,i) => `${participants[i]} : ${r}`).join("\n");

    // Copy the text inside the text field
    navigator.clipboard.writeText(rstr);
}

$(document).ready(function() {
    $('#result').hide();
    $('#done').hide();

    // handle get started button
    let getStarted = $("#get-started");
    getStarted.on("click", function(e) {
        e.preventDefault();
        scrollTo('#begin');
    });

    
    
    displayParticipents();

    // handle the participant table
    let participantName = $("#participant-name");
    let participantSubmit = $("#participant-add");
    let handleAddParticipant = () => {
        let name = participantName.val();
        if (name.length > 0) {
            participants.push(name);
            participantName.val("");
            displayParticipents();
        }
    };
    participantSubmit.on("click", function(e) {
        e.preventDefault();
        handleAddParticipant();
    });
    participantName.on("keypress", function(e) {
        if (e.which === 13) {
            e.preventDefault();
            handleAddParticipant();
        }
    });

    // Handle the form 
    let form = $("#creation-form");
    let name = $("#name");
    let start = $("#start");
    let end = $("#end");
    
    form.on("submit", function(e) {
        e.preventDefault();
        console.log("submitting form");

        let period = $('input[name="period"]:checked').val();

        if (period === "custom") {
            period = $("#custom-period-value").val();
            period = `${period}`
        }

        let data = {
            name: name.val(),
            start: start.val(),
            end: end.val(),
            participants: participants,
            period: period
        };

        if (data.name.length === 0) {
            alert("Please enter a name for the event");
            return;
        }
        if (data.participants.length === 0) {
            alert("Please add at least one participant");
            return;
        }
        if (data.start.length === 0) {
            alert("Please enter a start date");
            return;
        }
        if (data.end.length === 0) {
            alert("Please enter an end date");
            return;
        }

        console.log(data);
        
        res = urls(data);
        console.log(res);
        
        let result = $("#result");
        result.show();
        scrollTo("#result");
        progress = $("#progress");
        let progressVal = 0
        let progressFinish = ()=>{
            displayResults(res);
            $("#wait").hide();
        };
        let progressing = () => {
            progressVal += 1;
            progress.val(progressVal);
            if (progressVal < 100) {
                setTimeout(progressing, 33);
            } else {
                progressFinish();
            }
        }
        setTimeout(progressing, 33);
    });

});