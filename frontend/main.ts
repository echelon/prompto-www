/**
 * prompto-www frontend
 */
import $ = require('jquery');

$(function() {
  $('form').on('submit', function(e) {
    let filename = $('#file').val();
    console.log('form submit');
    console.log(e);
    //let files = drop.getAcceptedFiles();
    //console.log('accepted files', files);
    //files = drop.getRejectedFiles();
    //console.log('rejected files', files);
    //files = drop.getQueuedFiles();
    //console.log('queued files', files);
    e.preventDefault();

		let file = getFile();

		let formData = new FormData();
		formData.append('file', file);

		xhrUpload(formData);

    //drop.processQueue();
  });

});

function xhrUpload(formData: FormData) {
	var xhr = new XMLHttpRequest();
	xhr.onreadystatechange = function(){
		if (this.readyState == 4 && this.status == 200){
			console.log('xhr state change');
			console.log(this.response, typeof this.response);

			//this.response is what you're looking for
			//handler(this.response);
			var img = document.getElementById('img');
			var url = window.URL || window.webkitURL;
			let url2 = url.createObjectURL(this.response);
			console.log('url', url);
			console.log('url2', url2);
			img.src = url2;

		}
	}

	xhr.open('POST', '/upload');
	xhr.responseType = 'blob';

	xhr.send(formData); 
}

function getFile() : File {
	let $file = $('#file')[0];
	return $file.files[0];
}

window.xhrUpload = xhrUpload;
window.getFile = getFile;

