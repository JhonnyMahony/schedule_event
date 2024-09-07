using System;
using System.Data.SQLite;
using System.Threading;
using System.Threading.Tasks;
using Telegram.Bot;
using Telegram.Bot.Polling;
using Telegram.Bot.Types;
using Telegram.Bot.Types.Enums;
using Telegram.Bot.Types.ReplyMarkups;

var botClient = new TelegramBotClient("6432678822:AAHWr9V_scjs0sQpEHK8gLunU4LBtJoQk54");
var dbConnection = new SQLiteConnection("Data Source=events.db;Version=3;");
string currentStep = "";
string eventName = "";
string eventDescription = "";
string eventDate = "";
string userId = "";
int reminderTime = 60; // 1 hour by default

dbConnection.Open();
CreateDatabase();

var cts = new CancellationTokenSource();
var receiverOptions = new ReceiverOptions
{
    AllowedUpdates = { }
};

botClient.StartReceiving(HandleUpdateAsync, HandleErrorAsync, receiverOptions, cancellationToken: cts.Token);

Console.WriteLine("Бот запущений...");
Console.ReadLine();
cts.Cancel();

async Task HandleUpdateAsync(ITelegramBotClient botClient, Update update, CancellationToken cancellationToken)
{
    if (update.Type == UpdateType.Message && update.Message.Text != null)
    {
        var message = update.Message;
        userId = message.From.Id.ToString();

        switch (message.Text)
        {
            case "/start":
                var replyKeyboard = new ReplyKeyboardMarkup(new[]
                {
                    new KeyboardButton[] { "Запланувати подію", "Нагадування" },
                })
                {
                    ResizeKeyboard = true
                };

                await botClient.SendTextMessageAsync(message.Chat.Id, "Вітаю! Я допоможу вам запланувати події.", replyMarkup: replyKeyboard);
                break;

            case "Нагадування":
                await botClient.SendTextMessageAsync(message.Chat.Id, "За скільки часу нагадати про подію?", replyMarkup: GetReminderInlineKeyboard());
                break;

            case "Запланувати подію":
                await botClient.SendTextMessageAsync(message.Chat.Id, "Введіть назву події:");
                currentStep = "назва";
                break;

            default:
                await HandleEventCreation(message);
                break;
        }
    }
    else if (update.Type == UpdateType.CallbackQuery)
    {
        var callbackQuery = update.CallbackQuery;
        if (callbackQuery.Data != null)
        {
            switch (callbackQuery.Data)
            {
                case "remind_1h":
                    reminderTime = 60;
                    await botClient.AnswerCallbackQueryAsync(callbackQuery.Id, "Нагадування за 1 годину встановлено.");
                    break;

                case "remind_12h":
                    reminderTime = 720;
                    await botClient.AnswerCallbackQueryAsync(callbackQuery.Id, "Нагадування за 12 годин встановлено.");
                    break;

                case "remind_24h":
                    reminderTime = 1440;
                    await botClient.AnswerCallbackQueryAsync(callbackQuery.Id, "Нагадування за 24 години встановлено.");
                    break;
            }
        }
    }
}

InlineKeyboardMarkup GetReminderInlineKeyboard()
{
    return new InlineKeyboardMarkup(new[]
    {
        new[]
        {
            InlineKeyboardButton.WithCallbackData("1 година", "remind_1h"),
            InlineKeyboardButton.WithCallbackData("12 годин", "remind_12h"),
            InlineKeyboardButton.WithCallbackData("24 години", "remind_24h"),
        }
    });
}

async Task HandleEventCreation(Message message)
{
    var chatId = message.Chat.Id;
    switch (currentStep)
    {
        case "назва":
            eventName = message.Text;
            await botClient.SendTextMessageAsync(chatId, "Опишіть подію:");
            currentStep = "опис";
            break;

        case "опис":
            eventDescription = message.Text;
            await botClient.SendTextMessageAsync(chatId, "Введіть дату та час початку події (дд.мм.рррр гг:хх):");
            currentStep = "дата";
            break;

        case "дата":
            if (DateTime.TryParse(message.Text, out DateTime parsedDate))
            {
                if (parsedDate > DateTime.Now)
                {
                    eventDate = parsedDate.ToString();
                    SaveEventToDatabase(userId, eventName, eventDescription, eventDate, reminderTime);
                    await botClient.SendTextMessageAsync(chatId, "Подію успішно заплановано!");
                    ResetEventCreation();
                }
                else
                {
                    await botClient.SendTextMessageAsync(chatId, "Дата події повинна бути в майбутньому.");
                }
            }
            else
            {
                await botClient.SendTextMessageAsync(chatId, "Некоректний формат дати. Введіть ще раз.");
            }
            break;
    }
}

void SaveEventToDatabase(string userId, string name, string description, string date, int reminderTime)
{
    string insertEventQuery = @"INSERT INTO events (user_id, name, description, date, reminder_time)
                                VALUES (@userId, @name, @description, @date, @reminderTime)";
    using (var command = new SQLiteCommand(insertEventQuery, dbConnection))
    {
        command.Parameters.AddWithValue("@userId", userId);
        command.Parameters.AddWithValue("@name", name);
        command.Parameters.AddWithValue("@description", description);
        command.Parameters.AddWithValue("@date", date);
        command.Parameters.AddWithValue("@reminderTime", reminderTime);
        command.ExecuteNonQuery();
    }
}

// Функція для періодичного нагадування
void StartReminderChecker()
{
    Task.Run(async () =>
    {
        while (true)
        {
            CheckAndSendReminders();
            await Task.Delay(60000); // Перевірка кожні 60 секунд
        }
    });
}

void CheckAndSendReminders()
{
    string getEventsQuery = @"SELECT id, user_id, name, date, reminder_time FROM events WHERE date > @currentTime";
    using (var command = new SQLiteCommand(getEventsQuery, dbConnection))
    {
        command.Parameters.AddWithValue("@currentTime", DateTime.Now.ToString());

        using (var reader = command.ExecuteReader())
        {
            while (reader.Read())
            {
                var eventId = reader.GetInt32(0);
                var userId = reader.GetString(1);
                var eventName = reader.GetString(2);
                var eventDate = DateTime.Parse(reader.GetString(3));
                var reminderTime = reader.GetInt32(4);

                if (eventDate.AddMinutes(-reminderTime) <= DateTime.Now)
                {
                    // Надсилаємо повідомлення користувачу
                    botClient.SendTextMessageAsync(userId, $"Нагадування: ваша подія '{eventName}' почнеться о {eventDate}.");
                }
            }
        }
    }
}

void CreateDatabase()
{
    string createTableQuery = @"CREATE TABLE IF NOT EXISTS events (
                                id INTEGER PRIMARY KEY AUTOINCREMENT,
                                user_id TEXT,
                                name TEXT,
                                description TEXT,
                                date TEXT,
                                reminder_time INTEGER)";
    using (var command = new SQLiteCommand(createTableQuery, dbConnection))
    {
        command.ExecuteNonQuery();
    }
}

Task HandleErrorAsync(ITelegramBotClient botClient, Exception exception, CancellationToken cancellationToken)
{
    Console.WriteLine(exception.Message);
    return Task.CompletedTask;
}

void ResetEventCreation()
{
    currentStep = "";
    eventName = "";
    eventDescription = "";
    eventDate = "";
    userId = "";
    reminderTime = 60; // За замовчуванням 1 година
}
