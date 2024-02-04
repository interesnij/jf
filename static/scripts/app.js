
//пока сокеты оставим неактивными
//connect() 

check_first_load();

on('body', 'click', '.menu-button', function() {
  block = this.nextElementSibling;
  if (this.classList.contains("open")) {
    this.classList.remove("open");
    block.style.opacity = "1";
    block.style.visibility = "visible";
  }
  if (this.classList.contains("open")) {
    this.classList.add("open")
    block.style.opacity = "0";
    block.style.visibility = "hidden";
  }
});

on('body', 'click', '#logg', function() {
  _this = this;
  form = _this.parentElement.parentElement; 
  response1 = form.querySelector(".api_response1");
  response2 = form.querySelector(".api_response2");

  if (!form.querySelector("#id_email").value){
    form.querySelector("#id_email").style.border = "1px #FF0000 solid";
    response1.innerHTML = "Enter your email!";
    response1.classList.add("error");
    return
  }
  else if (!form.querySelector("#id_password").value){
    form.querySelector("#id_password").style.border = "1px #FF0000 solid";
    response2.innerHTML = "Enter the password!";
    response2.classList.add("error")
    return
  }
  else {
    _this.disabled = true;
  }

  form_data = new FormData(form);
  link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link.open( 'POST', "https://backend.juslaw.com/api/v1/auth/login/", true );

  link.onreadystatechange = function () {
  if ( link.readyState == 4 && link.status == 200 ) {
    localStorage.setItem("request_data", link.response);
    window.location.href = "/";
  }

  else {
    _this.disabled = false;
  }};
  link.send(form_data);
});

on('body', 'click', '#signup', function() {
  _this = this;
  form = _this.parentElement;
  username = form.querySelector("#id_email");
  response1 = form.querySelector(".api_response1");
  response2 = form.querySelector(".api_response2");
  if (!username.value){
    username.style.border = "1px #FF0000 solid";
    toast_error("Enter your email!");
    return
  } else if (!form.querySelector("#id_password").value){
    form.querySelector("#id_password").style.border = "1px #FF0000 solid";
    toast_error("Enter your password!");
    return
  }
  else {
    this.disabled = true
  }

  form_data = new FormData(form);
  link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link.open( 'POST', "/signup/", true );

  link.onreadystatechange = function () {
  if ( link.readyState == 4 && link.status == 200 ) {
    localStorage.setItem("request_data", link.response);
    window.location.href = "/";
    }
  else {
    _this.disabled = false;
    response.style.display = "block";
    response.innerHTML = "not ok";
    response.classList.add("error");
  }};
  link.send(form_data);
});

on('body', 'click', '.ajax', function(event) {
    event.preventDefault();
    if (this.classList.contains("navbar__item")) {
        try {
          document.body.querySelector("header__title").innerHTML = this.innerHTML;
        } 
        catch {}
    }
    ajax_get_reload(this.getAttribute("href"), true, 2);
});

