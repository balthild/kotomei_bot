import re
import random
from telegram.ext import Updater, MessageHandler, BaseFilter, Filters
from . import config


class FilterPr(BaseFilter):
    def __init__(self):
        self.regex = re.compile(r'/(pr)+(@kotomei(_bot)?)?')

    def filter(self, message):
        return bool(message.text and self.regex.match(message.text) is not None)


emoticons = ['⁄(⁄ ⁄•⁄ω⁄•⁄ ⁄)⁄', 'ヾ(*´∀ ˋ*)ﾉ', '(≧ ﹏ ≦)']


def pr(bot, update):
    if random.randint(1, 100) > 95:
        update.message.reply_text(random.choice(emoticons))
    else:
        update.message.reply_text('( >  < )')


def debug(bot, update):
    print(update.message.text)


def main():
    # logging.basicConfig(format='%(asctime)s - %(name)s - %(levelname)s - %(message)s', level=logging.INFO)

    updater = Updater(config.TOKEN)

    updater.dispatcher.add_handler(MessageHandler(FilterPr(), pr))
    # updater.dispatcher.add_handler(MessageHandler(Filters.all, debug))

    updater.start_polling()
    updater.idle()


if __name__ == '__main__':
    main()
