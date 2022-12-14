#include <oboe/Oboe.h>
#include <math.h>
#include <deque>
#include <pthread.h>

// I got link problem with std::mutex, so use pthread instead
class CThreadLock
{
public:
    CThreadLock();
    virtual ~CThreadLock();

    void Lock();
    void Unlock();

private:
    pthread_mutex_t mutexlock;
};

CThreadLock::CThreadLock()
{
    // init lock here
    pthread_mutex_init(&mutexlock, 0);
}

CThreadLock::~CThreadLock()
{
    // deinit lock here
    pthread_mutex_destroy(&mutexlock);
}
void CThreadLock::Lock()
{
    // lock
    pthread_mutex_lock(&mutexlock);
}
void CThreadLock::Unlock()
{
    // unlock
    pthread_mutex_unlock(&mutexlock);
}

class Player : public oboe::AudioStreamDataCallback
{
public:
    Player(int channels, int sample_rate)
    {
        this->channels = channels;
        oboe::AudioStreamBuilder builder;
        // The builder set methods can be chained for convenience.
        builder.setSharingMode(oboe::SharingMode::Exclusive)
            ->setPerformanceMode(oboe::PerformanceMode::LowLatency)
            ->setChannelCount(channels)
            ->setSampleRate(sample_rate)
            ->setFormat(oboe::AudioFormat::Float)
            ->setDataCallback(this)
            ->openManagedStream(outStream);
        // Typically, start the stream after querying some stream information, as well as some input from the user
        outStream->requestStart();
    }

    ~Player() {
        outStream->requestStop();
    }

    oboe::DataCallbackResult onAudioReady(oboe::AudioStream *oboeStream, void *audioData, int32_t numFrames) override
    {
        float *floatData = (float *)audioData;
        int i = 0;
        mtx.Lock();
        auto n = channels * numFrames;
        for (; i < n && i < (int)buffer.size(); ++i, ++floatData)
        {
            *floatData = buffer.front();
            buffer.pop_front();
        }
        mtx.Unlock();
        for (; i < n; ++i, ++floatData)
        {
            *floatData = 0;
        }
        return oboe::DataCallbackResult::Continue;
    }

    void push(const float *v, int n)
    {
        mtx.Lock();
        for (auto i = 0; i < n; ++i, ++v)
            buffer.push_back(*v);
        // in case memory overuse
        if (buffer.size() > 48 * 1024 * 120)
            buffer.clear();
        mtx.Unlock();
    }

private:
    oboe::ManagedStream outStream;
    int channels;
    std::deque<float> buffer;
    CThreadLock mtx;
};

extern "C"
{
    void *create_oboe_player(int channels, int sample_rate)
    {
        return new Player(channels, sample_rate);
    }

    void push_oboe_data(void *player, const float* v, int n)
    {
        static_cast<Player *>(player)->push(v, n);
    }

    void destroy_oboe_player(void *player)
    {
        delete static_cast<Player *>(player);
    }
}