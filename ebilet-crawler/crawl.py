import requests
import datetime
import re
import time
from bs4 import BeautifulSoup
import tags

CHECK_URL = "/view/eybis/tnmGenel/tcddWebContent.jsf"
BASE_URL = "https://ebilet.tcddtasimacilik.gov.tr"


def is_valid_date(tarih):
    # TODO: Implement
    return True


def get_post_data(
    nereden: str, nereye: str, tarih: str, face_view_state, yolcu_sayisi=1
):
    """
    Post metodu için uygun veriyi döndürür.
    """
    if nereden not in tags.tags and nereye not in tags.tags and not is_valid_date:
        raise
    return {
        "javax.faces.partial.ajax": "true",
        "javax.faces.source": "btnSeferSorgula",
        "javax.faces.partial.execute": "btnSeferSorgula biletAramaForm",
        "javax.faces.partial.render": "msg biletAramaForm",
        "btnSeferSorgula": "btnSeferSorgula",
        "biletAramaForm": "biletAramaForm",
        "tipradioIslemTipi": "0",
        "nereden": nereden,
        "trCalGid_input": tarih,
        "tipradioSeyehatTuru": "1",
        "nereye": nereye,
        "syolcuSayisi_input": yolcu_sayisi,
        "javax.faces.ViewState": face_view_state,
    }


def uygunKoltukBak(nereden: str, nereye: str, tarih: str):
    """ """
    session = requests.Session()
    get_url = session.get(BASE_URL + CHECK_URL)
    cookies = get_url.cookies
    unix_timestamp = str(int(time.mktime(datetime.datetime.now().timetuple())))
    cookies["yolcuTabId"] = "yolcuTabId" + unix_timestamp
    headers = {
        "Host": "ebilet.tcddtasimacilik.gov.tr",
        "User-Agent": "Mozilla/5.0 (X11; Linux x86_64; rv:102.0) Gecko/20100101 Firefox/102.0",
        "Accept": "application/xml, text/xml, */*; q=0.01",
        "Accept-Language": "en-US,en;q=0.5",
        "Accept-Encoding": "gzip, deflate, br",
        "Content-Type": "application/x-www-form-urlencoded; charset=UTF-8",
        "Faces-Request": "partial/ajax",
        "X-Requested-With": "XMLHttpRequest",
        "Content-Length": "437",
        "Origin": "https://ebilet.tcddtasimacilik.gov.tr",
        "Connection": "keep-alive",
        "Referer": "https://ebilet.tcddtasimacilik.gov.tr/view/eybis/tnmGenel/tcddWebContent.jsf",
        "Sec-Fetch-Dest": "empty",
        "Sec-Fetch-Mode": "cors",
        "Sec-Fetch-Site": "same-origin",
        "Sec-GPC": "1",
    }
    get_soup = BeautifulSoup(get_url.content, features="xml")
    faces_view_state = get_soup.find("input", {"name": "javax.faces.ViewState"})
    if faces_view_state and faces_view_state.has_attr("value"):
        faces_view_state = str(faces_view_state["value"])
    else:
        return False
    data = get_post_data(nereden, nereye, tarih, face_view_state=faces_view_state)
    x = session.post(
        BASE_URL + CHECK_URL,
        data=data,
        headers=headers,
        cookies=cookies,
        allow_redirects=True,
    )
    soup = BeautifulSoup(x.content, features="xml")
    redirect_url = ""
    try:
        redirect_url = soup.find("redirect")["url"]
    except TypeError:
        return
    x = session.get(BASE_URL + redirect_url)
    soup = BeautifulSoup(x.content, features="xml")
    seferler = soup.findAll(
        "tbody",
        id="mainTabView:gidisSeferTablosu_data",
        class_="ui-datatable-data ui-widget-content",
    )
    return seferler


def printSeferler(seferler):
    if not seferler:
        print("Tren bulunamadı")
        return
    for sefer in seferler[0]:
        print("Kalkış")
        print(sefer.contents[0].contents[0].text)
        print(sefer.contents[0].span.text)
        print("\nSüre")
        print(sefer.contents[1].text)
        print("\nVarış")
        print(sefer.contents[2].contents[0].text)
        print(sefer.contents[2].span.text)
        print("Secenekler")
        for s in sefer.contents[4]:
            try:
                q = s.findAll("option")
            except:
                q = []
            if q != []:
                print(q[0].text)
        print("Fiyatlar")
        print(sefer.contents[5].text)
        print("----")


if __name__ == "__main__":
    seferler = uygunKoltukBak("Karaman", "Ankara Gar", "22.07.2022")
    printSeferler(seferler)
    seferler = uygunKoltukBak("Karaman", "Bahçeli+(Km.755+290+S)", "22.07.2022")
    printSeferler(seferler)
