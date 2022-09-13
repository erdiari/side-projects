import click
from crawl import is_valid_date, printSeferler, uygunKoltukBak
import tags
from threading import Timer
from threading import Lock


class RepeatingTimer(object):
    """
    https://stackoverflow.com/questions/15584928/fast-and-precise-python-repeating-timer
    """

    def __init__(self, interval, function, *args, **kwargs):
        super(RepeatingTimer, self).__init__()
        self.args = args
        self.kwargs = kwargs
        self.function = function
        self.interval = interval
        self.start()

    def start(self):
        self.callback()

    def stop(self):
        self.interval = False

    def callback(self):
        if self.interval:
            self.function(*self.args, **self.kwargs)
            Timer(
                self.interval,
                self.callback,
            ).start()


@click.group()
def main():
    pass


@main.command()
@click.option("--nereden", prompt="Nereden?", type=click.Choice(tags.tags, case_sensitive=False))
@click.option("--nereye", prompt="Nereye?", type=click.Choice(tags.tags, case_sensitive=False))
@click.option("--tarih", prompt="Ne zaman (gg.aa.yyyy)?", type=str)
def bir_kere(nereden, nereye, tarih):
    if is_valid_date(tarih):
        seferler = uygunKoltukBak(nereden, nereye, tarih)
        printSeferler(seferler)
    else:
        print("Tarih gg.aa.yyyy formatında ve geçerli bir tarih olmalı.")


def _kontrol_ve_print(nereden, nereye, tarih):
    seferler = uygunKoltukBak(nereden, nereye, tarih)
    printSeferler(seferler)


@main.command()
@click.option("--nereden", prompt="Nereden?", type=click.Choice(tags.tags,case_sensitive=False) )
@click.option("--nereye", prompt="Nereye?", type=click.Choice(tags.tags, case_sensitive=False))
@click.option("--tarih", prompt="Ne zaman (gg.aa.yyyy)?", type=str)
@click.option("--kac-sureli", prompt="Ne kadar sürede yeniden bak (saniye)", type=int)
def duzenli(nereden, nereye, tarih, kac_sureli):
    if is_valid_date(tarih):
        listLock = Lock()
        rt = RepeatingTimer(kac_sureli, _kontrol_ve_print, nereden, nereye, tarih)
    else:
        print("Tarih gg.aa.yyyy formatında ve geçerli bir tarih olmalı.")


if __name__ == "__main__":
    main()
