on('body', 'click', '.register_final_client_btn', function() {
    if (localStorage.getItem("reg_step_1") === null) {
      back_register_1_step_low("/auth/register/client");
    }
  
    _tObject = localStorage.getItem('reg_step_1');
    tObject = JSON.parse(_tObject);
  
    form = document.body.querySelector(".js_form");
    _first_name = form.querySelector("#id_first_name");
    _last_name = form.querySelector("#id_last_name");
    _phone = form.querySelector("#id_phone");
    _attachments = form.querySelector("#id_attachments");
    email = tObject.email;
    password1 = tObject.password;
    password2 = tObject.password;  
  
    _jurisdictions_countries = form.querySelectorAll(".country");
    _jurisdictions_states = form.querySelectorAll(".state");
    _jurisdictions_numbers = form.querySelectorAll(".id_number");
    _jurisdictions_years = form.querySelectorAll(".id_year");
  
    let is_ok = validate (
      _first_name,
      _last_name,
      _phone,
      _attachments, 
      _jurisdictions_countries[0],
      _jurisdictions_states[0],
      null, null, null,
      _jurisdictions_years[0],
      _jurisdictions_numbers[0]
    );
  
    console.log("is_ok", is_ok);
    if (!is_ok) {
      return;
    }
  
    jurisdictions = [];
    for (let i = 0; i < _jurisdictions_numbers.length; i++) {
      //try {
        let country, state, number, year;
        country = _jurisdictions_countries[i].getAttribute("data-pk");
        state = _jurisdictions_states[i].getAttribute("data-pk");
        number = _jurisdictions_numbers[i].value;
        year = _jurisdictions_years[i].value;
        jurisdictions.push({
          country: country, 
          state: state,
          number: number, 
          year: year
        });
      //} catch { null }
    }; 
    
    ///// 
      file_data = get_file_data(_attachments.files);
      _link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
      _link.open( 'POST', "https://k.juslaw.online/classic_create/", true );
      _link.onreadystatechange = function () {
      if ( _link.readyState == 4 && _link.status == 200 ) {
          data = JSON.parse(_link.responseText);
          files = data["files"];
      }};
      _link.send(file_data);
      ///// 
  
    is_disciplined = false;
    if(form.querySelector('#disciplined_true').checked) {
      is_disciplined = true;
    }
  
    fObject = {
      "email": email,
      "password1": password1,
      "password2": password2,
      "first_name": _first_name.value,
      "last_name": _last_name.value,
      "phone": _phone.value,
      "is_disciplined": is_disciplined,
      "practice_jurisdictions": jurisdictions,
      "registration_attachments": files
    }
    
    localStorage.setItem('reg_step_2', JSON.stringify(fObject));
    fetch('https://backend.juslaw.com/api/v1/users/attorneys/', {
      method: 'POST',
      headers: {
        'Accept': 'application/json, text/plain, */*',
        'Content-Type': 'application/json'
      },
      body: JSON.stringify(fObject)
    }).then(res => res.json())
      .then(res => show_law_reg_success_modal());
  });